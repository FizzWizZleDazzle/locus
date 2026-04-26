//! CPU bytecode VM + Cartesian-product enumeration via rayon.
//!
//! Acts as the M1 fallback when the GPU is unavailable, and as the source of
//! truth for cross-validation against the GPU executor.

use rayon::prelude::*;
use xxhash_rust::xxh3::xxh3_64;

use crate::gpu::bytecode::run as run_program;
use crate::gpu::compile::{EvalStep, Plan};

/// Each entry is the resolved `i32` value for every slot in `Plan::var_names`.
pub type SurvivorRow = Vec<i32>;

/// Enumerate all valid combinations on CPU, dedup by sampler-tuple hash.
/// Symmetric duplicates are NOT collapsed here (same hash → same row); the
/// enumerator's post-render `DashSet` collapses them on `(question, answer)`.
///
/// Returns `(survivor_rows, total_kept_after_filter)`. Survivor rows already
/// deduped — caller only needs to render LaTeX.
pub fn run_cpu(plan: &Plan, target: usize) -> (Vec<SurvivorRow>, usize) {
    if plan.total_combos == 0 || plan.var_names.is_empty() {
        return (Vec::new(), 0);
    }

    let radixes: Vec<usize> = plan.sampler_slots.iter().map(|s| s.values.len()).collect();
    let total = plan.total_combos as usize;
    let n_vars = plan.var_names.len();

    // Parallel map: for each Cartesian index, emit (hash, row) if constraints pass.
    let pairs: Vec<(u64, SurvivorRow)> = (0..total)
        .into_par_iter()
        .filter_map(|tid| {
            let mut sampler_idx = vec![0usize; radixes.len()];
            let mut t = tid;
            for (i, &r) in radixes.iter().enumerate() {
                sampler_idx[i] = t % r;
                t /= r;
            }

            let mut row: SurvivorRow = vec![0; n_vars];

            for step in &plan.eval_steps {
                match step {
                    EvalStep::Sampler {
                        sampler_idx: si,
                        var_slot,
                    } => {
                        let s = &plan.sampler_slots[*si as usize];
                        row[*var_slot as usize] = s.values[sampler_idx[*si as usize]];
                    }
                    EvalStep::Derived { var_slot, program } => match run_program(program, &row) {
                        Ok(v) => row[*var_slot as usize] = v,
                        Err(_) => return None,
                    },
                }
            }

            for c in &plan.constraints {
                match run_program(c, &row) {
                    Ok(v) => {
                        if v == 0 {
                            return None;
                        }
                    }
                    Err(_) => return None,
                }
            }

            // Hash on dedup_slots (sampler tuple). Distinct sampler tuples
            // get distinct hashes regardless of derived collisions.
            let mut hash_buf = Vec::with_capacity(plan.dedup_slots.len() * 4);
            for &slot in &plan.dedup_slots {
                hash_buf.extend_from_slice(&row[slot as usize].to_le_bytes());
            }
            let h = xxh3_64(&hash_buf);
            Some((h, row))
        })
        .collect();

    let total_kept = pairs.len();

    // Dedup by hash. Stable: keep first by Cartesian index. Sort by index baked
    // into the input order via collect — par_iter preserves index order in the
    // output Vec.
    let mut seen = std::collections::HashSet::with_capacity(pairs.len());
    let mut out = Vec::with_capacity(target.min(pairs.len()));
    for (h, row) in pairs {
        if seen.insert(h) {
            out.push(row);
            if out.len() >= target {
                break;
            }
        }
    }

    (out, total_kept)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu::compile::compile;
    use std::collections::BTreeMap;

    #[test]
    fn enumerate_quadratic_roots_keeps_distinct_samplers() {
        // r1, r2 ∈ {1, 2, 3}, r1 != r2. GPU/CPU executor hashes on sampler
        // tuple only — symmetric pairs (1,2)↔(2,1) survive both. Post-render
        // dedup in the enumerator collapses them when (question, answer)
        // matches; this lower-level test sees the un-collapsed 6.
        let mut vars = BTreeMap::new();
        vars.insert("r1".into(), "integer(1, 3)".into());
        vars.insert("r2".into(), "integer(1, 3)".into());
        vars.insert("s".into(), "r1 + r2".into());
        vars.insert("p".into(), "r1 * r2".into());
        let constraints = vec!["r1 != r2".into()];
        let plan = compile(&vars, &constraints).unwrap();

        let (rows, total_kept) = run_cpu(&plan, 1000);

        // 3*3 = 9 raw, 6 pass r1!=r2.
        assert_eq!(total_kept, 6);
        assert_eq!(rows.len(), 6);
    }

    #[test]
    fn no_constraints_no_dedup_when_unique() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "integer(1, 5)".into());
        vars.insert("b".into(), "a * 2".into());
        let plan = compile(&vars, &[]).unwrap();
        let (rows, kept) = run_cpu(&plan, 100);
        assert_eq!(kept, 5);
        assert_eq!(rows.len(), 5);
    }
}
