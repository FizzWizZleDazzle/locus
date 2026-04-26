//! Top-level enumeration driver. Builds a `Plan` from a spec/variant, runs the
//! chosen executor (CPU rayon or GPU wgpu), then renders each unique numeric
//! tuple into a `ProblemOutput` via the existing template/answer modules.

use std::collections::BTreeMap;

use dashmap::DashSet;
use rayon::prelude::*;
use xxhash_rust::xxh3::xxh3_128;

use crate::error::DslError;
use crate::gpu::compile::{compile, CompileError, Plan};
use crate::gpu::cpu_exec::run_cpu;
use crate::resolver::{resolve_with_preset, VarMap};
use crate::spec::{ProblemSpec, Variant};
use crate::{answer, sampler, template, ProblemOutput};

/// Executor selection. `Auto` prefers GPU when available, else CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Executor {
    Auto,
    Cpu,
    Gpu,
}

/// Try to enumerate up to `target` unique problems for a top-level spec.
///
/// Iterates each variant and concatenates results.
/// Returns `Ok(None)` if any variant contains something we can't compile
/// (caller should fall back to legacy rejection sampling).
pub fn enumerate(
    spec: &ProblemSpec,
    target: usize,
    executor: Executor,
) -> Result<Option<Vec<ProblemOutput>>, DslError> {
    let mut all = Vec::new();
    for variant in &spec.variants {
        match enumerate_one(spec, variant, target, executor)? {
            Some(rows) => all.extend(rows),
            None => return Ok(None),
        }
    }
    Ok(Some(all))
}

fn enumerate_one(
    spec: &ProblemSpec,
    variant: &Variant,
    target: usize,
    executor: Executor,
) -> Result<Option<Vec<ProblemOutput>>, DslError> {
    let plan = match compile(&variant.variables, &variant.constraints) {
        Ok(p) => p,
        Err(CompileError::Unsupported(_)) => return Ok(None),
        Err(e) => return Err(DslError::ExpressionParse(e.to_string())),
    };

    // GPU launch + buffer setup costs ~1ms per YAML. For small Cartesian
    // products (< 100k) the CPU rayon path is faster end-to-end. Cross over
    // matches what the wgpu shader compile + readback overhead measures at.
    const GPU_MIN_COMBOS: u64 = 100_000;

    let rows = match executor {
        Executor::Cpu => run_cpu(&plan, target).0,
        Executor::Gpu => run_gpu_or_fail(&plan, target)?,
        Executor::Auto => {
            if plan.total_combos >= GPU_MIN_COMBOS {
                match try_gpu(&plan, target) {
                    Some(rows) => rows,
                    None => run_cpu(&plan, target).0,
                }
            } else {
                run_cpu(&plan, target).0
            }
        }
    };

    // Render survivors in parallel, applying CPU constraints, then dedupe on
    // the canonical (question, answer) hash. CPU dedup is the safety net that
    // catches symmetric duplicates where the GPU couldn't dedup numerically
    // (e.g. when the answer var was unhoistable).
    let seen: DashSet<u128> = DashSet::new();
    let outs: Vec<ProblemOutput> = rows
        .par_iter()
        .filter_map(|row| render_row(spec, variant, &plan, row).ok().flatten())
        .filter(|p| {
            let key = canonical_key(&p.question_latex, &p.answer_key);
            seen.insert(key)
        })
        .take_any(target)
        .collect();
    Ok(Some(outs))
}

fn canonical_key(question: &str, answer: &str) -> u128 {
    let mut buf = Vec::with_capacity(question.len() + answer.len() + 1);
    buf.extend_from_slice(answer.as_bytes());
    buf.push(0x1f);
    buf.extend_from_slice(question.as_bytes());
    xxh3_128(&buf)
}

#[cfg(feature = "gpu")]
fn try_gpu(plan: &Plan, target: usize) -> Option<Vec<crate::gpu::cpu_exec::SurvivorRow>> {
    crate::gpu::gpu_exec::run_gpu(plan, target).map(|(rows, _)| rows)
}

#[cfg(not(feature = "gpu"))]
fn try_gpu(_plan: &Plan, _target: usize) -> Option<Vec<crate::gpu::cpu_exec::SurvivorRow>> {
    None
}

#[cfg(feature = "gpu")]
fn run_gpu_or_fail(
    plan: &Plan,
    target: usize,
) -> Result<Vec<crate::gpu::cpu_exec::SurvivorRow>, DslError> {
    match crate::gpu::gpu_exec::run_gpu(plan, target) {
        Some((rows, _)) => Ok(rows),
        None => Err(DslError::Evaluation(
            "GPU not available or plan not GPU-eligible".into(),
        )),
    }
}

#[cfg(not(feature = "gpu"))]
fn run_gpu_or_fail(
    _plan: &Plan,
    _target: usize,
) -> Result<Vec<crate::gpu::cpu_exec::SurvivorRow>, DslError> {
    Err(DslError::Evaluation(
        "GPU executor requires --features gpu".into(),
    ))
}

fn render_row(
    spec: &ProblemSpec,
    variant: &Variant,
    plan: &Plan,
    row: &[i32],
) -> Result<Option<ProblemOutput>, DslError> {
    // Build preset VarMap from the sampler values only — derived/symbolic vars
    // get recomputed by resolve_with_preset so YAMLs with `f: a*x^2 + b*x` style
    // expressions get the correct symbolic string for template substitution.
    let mut presets: VarMap = BTreeMap::new();
    for s in &plan.sampler_slots {
        let name = &plan.var_names[s.var_slot as usize];
        presets.insert(name.clone(), row[s.var_slot as usize].to_string());
    }

    let vars = match resolve_with_preset(&variant.variables, &presets) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    // CPU constraints: ones that touch unhoisted vars (e.g. `det != 0`).
    // GPU constraints already applied during enumeration; these are the
    // remainder, evaluated against the fully-resolved VarMap.
    for c in &plan.cpu_constraints {
        match crate::resolver::eval_constraint_str(c, &vars) {
            Ok(true) => {}
            Ok(false) => return Ok(None),
            Err(_) => return Ok(None),
        }
    }

    let question_latex = match template::render(&variant.question, &vars) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    let answer_key = match answer::format(&vars, &variant.answer, variant.answer_type.as_deref()) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    let solution_latex = match &variant.solution {
        Some(steps) => match template::render_steps(steps, &vars) {
            Ok(s) => s,
            Err(_) => return Ok(None),
        },
        None => String::new(),
    };

    let answer_type = answer::infer_type(&answer_key, variant.answer_type.as_deref());
    let difficulty = variant.difficulty.as_ref().unwrap_or(&spec.difficulty);

    let question_image_url = match &variant.diagram {
        Some(d) => match crate::diagram::render(d, &vars) {
            Ok(s) => s,
            Err(_) => return Ok(None),
        },
        None => String::new(),
    };

    Ok(Some(ProblemOutput {
        question_latex,
        answer_key,
        solution_latex,
        difficulty: sampler::sample_difficulty(difficulty)?,
        main_topic: spec.topic.main.clone(),
        subtopic: spec.topic.sub.clone(),
        grading_mode: variant.mode.clone().unwrap_or_else(|| "equivalent".into()),
        answer_type,
        calculator_allowed: spec.calculator.clone().unwrap_or_else(|| "none".into()),
        question_image_url,
        time_limit_seconds: spec.time,
        variant_name: variant.name.clone(),
    }))
}
