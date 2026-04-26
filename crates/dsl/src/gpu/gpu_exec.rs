//! GPU executor — wgpu compute pipeline running the bytecode VM in
//! `program.wgsl`. Mirrors `cpu_exec` semantics; results must match.

use std::collections::HashSet;
use std::sync::OnceLock;

use bytemuck::Zeroable;

use crate::gpu::compile::{EvalStep, Plan};
use crate::gpu::cpu_exec::SurvivorRow;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuCfg {
    n_samplers: u32,
    n_steps: u32,
    n_constraints: u32,
    n_vars: u32,
    n_dedup_slots: u32,
    total_combos: u32,
    _pad0: u32,
    _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuStep {
    kind: u32,
    var_slot: u32,
    sampler_idx: u32,
    program_off: u32,
    program_pairs: u32,
    consts_off: u32,
    consts_count: u32,
    _pad0: u32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuConstraintRange {
    program_off: u32,
    program_pairs: u32,
    consts_off: u32,
    consts_count: u32,
}

struct GpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

static CTX: OnceLock<Option<GpuContext>> = OnceLock::new();

fn get_ctx() -> Option<&'static GpuContext> {
    CTX.get_or_init(|| init_ctx().ok()).as_ref()
}

fn init_ctx() -> Result<GpuContext, String> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12,
        ..Default::default()
    });

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..Default::default()
    }))
    .ok_or("no GPU adapter")?;

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("locus-enum"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits {
                max_storage_buffer_binding_size: 1 << 30,
                max_buffer_size: 1 << 30,
                max_storage_buffers_per_shader_stage: 16,
                ..wgpu::Limits::default()
            },
            ..Default::default()
        },
        None,
    ))
    .map_err(|e| format!("device: {e}"))?;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("enum_program"),
        source: wgpu::ShaderSource::Wgsl(include_str!("program.wgsl").into()),
    });

    let entries: Vec<wgpu::BindGroupLayoutEntry> = (0..12)
        .map(|i| wgpu::BindGroupLayoutEntry {
            binding: i,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: if i == 0 {
                    wgpu::BufferBindingType::Uniform
                } else if i >= 9 {
                    wgpu::BufferBindingType::Storage { read_only: false }
                } else {
                    wgpu::BufferBindingType::Storage { read_only: true }
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect();

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("enum_bgl"),
        entries: &entries,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("enum_pl"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("enum_pipe"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("enumerate"),
        compilation_options: Default::default(),
        cache: None,
    });

    Ok(GpuContext {
        device,
        queue,
        pipeline,
        bind_group_layout,
    })
}

/// True if we can execute `plan` on GPU. Returns false for shapes the kernel
/// can't handle in M2 (too many slots, too many samplers, etc.).
pub fn is_supported(plan: &Plan) -> bool {
    plan.var_names.len() <= 32
        && plan.sampler_slots.len() <= 16
        && plan.total_combos <= 16_000_000
        && plan.total_combos > 0
}

/// Try to enumerate on GPU. Returns `None` if the GPU is unavailable.
pub fn run_gpu(plan: &Plan, target: usize) -> Option<(Vec<SurvivorRow>, usize)> {
    if !is_supported(plan) {
        return None;
    }
    let ctx = get_ctx()?;
    Some(dispatch(ctx, plan, target))
}

fn dispatch(ctx: &GpuContext, plan: &Plan, target: usize) -> (Vec<SurvivorRow>, usize) {
    let total = plan.total_combos as u32;
    let n_vars = plan.var_names.len() as u32;
    let n_samplers = plan.sampler_slots.len() as u32;

    // Flatten sampler values
    let mut sampler_offsets: Vec<u32> = Vec::with_capacity(plan.sampler_slots.len());
    let mut sampler_values: Vec<i32> = Vec::new();
    let mut radixes: Vec<u32> = Vec::with_capacity(plan.sampler_slots.len());
    for s in &plan.sampler_slots {
        sampler_offsets.push(sampler_values.len() as u32);
        radixes.push(s.values.len() as u32);
        sampler_values.extend_from_slice(&s.values);
    }

    // Flatten programs
    let mut code: Vec<u32> = Vec::new();
    let mut consts_buf: Vec<i32> = Vec::new();
    let mut steps: Vec<GpuStep> = Vec::with_capacity(plan.eval_steps.len());

    for step in &plan.eval_steps {
        match step {
            EvalStep::Sampler {
                sampler_idx,
                var_slot,
            } => {
                steps.push(GpuStep {
                    kind: 0,
                    var_slot: *var_slot,
                    sampler_idx: *sampler_idx,
                    program_off: 0,
                    program_pairs: 0,
                    consts_off: 0,
                    consts_count: 0,
                    _pad0: 0,
                });
            }
            EvalStep::Derived { var_slot, program } => {
                let prog_off = code.len() as u32;
                let prog_pairs = (program.code.len() / 2) as u32;
                let consts_off = consts_buf.len() as u32;
                let consts_count = program.consts.len() as u32;
                code.extend_from_slice(&program.code);
                consts_buf.extend_from_slice(&program.consts);
                steps.push(GpuStep {
                    kind: 1,
                    var_slot: *var_slot,
                    sampler_idx: 0,
                    program_off: prog_off,
                    program_pairs: prog_pairs,
                    consts_off,
                    consts_count,
                    _pad0: 0,
                });
            }
        }
    }

    let mut constraint_ranges: Vec<GpuConstraintRange> = Vec::with_capacity(plan.constraints.len());
    for prog in &plan.constraints {
        let prog_off = code.len() as u32;
        let prog_pairs = (prog.code.len() / 2) as u32;
        let consts_off = consts_buf.len() as u32;
        let consts_count = prog.consts.len() as u32;
        code.extend_from_slice(&prog.code);
        consts_buf.extend_from_slice(&prog.consts);
        constraint_ranges.push(GpuConstraintRange {
            program_off: prog_off,
            program_pairs: prog_pairs,
            consts_off,
            consts_count,
        });
    }

    let dedup_idx: Vec<u32> = plan.dedup_slots.clone();

    let cfg = GpuCfg {
        n_samplers,
        n_steps: plan.eval_steps.len() as u32,
        n_constraints: plan.constraints.len() as u32,
        n_vars,
        n_dedup_slots: dedup_idx.len() as u32,
        total_combos: total,
        _pad0: 0,
        _pad1: 0,
    };

    use wgpu::util::DeviceExt;
    let mk_storage = |label: &str, data: &[u8]| -> wgpu::Buffer {
        ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage: wgpu::BufferUsages::STORAGE,
        })
    };

    // Pad zero-sized buffers since wgpu requires non-empty bindings.
    let constraint_data = if constraint_ranges.is_empty() {
        vec![GpuConstraintRange::zeroed()]
    } else {
        constraint_ranges
    };
    let dedup_data = if dedup_idx.is_empty() { vec![0u32] } else { dedup_idx };
    let consts_data = if consts_buf.is_empty() { vec![0i32] } else { consts_buf };
    let code_data = if code.is_empty() { vec![0u32] } else { code };
    let steps_data = if steps.is_empty() { vec![GpuStep::zeroed()] } else { steps };

    let cfg_buf = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("cfg"),
        contents: bytemuck::bytes_of(&cfg),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let radixes_buf = mk_storage("radixes", bytemuck::cast_slice(&radixes));
    let so_buf = mk_storage("sampler_offsets", bytemuck::cast_slice(&sampler_offsets));
    let sv_buf = mk_storage("sampler_values", bytemuck::cast_slice(&sampler_values));
    let steps_buf = mk_storage("steps", bytemuck::cast_slice(&steps_data));
    let code_buf = mk_storage("code", bytemuck::cast_slice(&code_data));
    let consts_buf_b = mk_storage("consts", bytemuck::cast_slice(&consts_data));
    let cr_buf = mk_storage("cr", bytemuck::cast_slice(&constraint_data));
    let dd_buf = mk_storage("dedup_idx", bytemuck::cast_slice(&dedup_data));

    let rows_size = (total as u64) * (n_vars as u64) * 4;
    let valid_size = (total as u64) * 4;
    let hash_size = (total as u64) * 4;

    let rows_buf = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("rows"),
        size: rows_size.max(4),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let valid_buf = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("valid"),
        size: valid_size.max(4),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let hash_buf = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("hash"),
        size: hash_size.max(4),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let rows_stg = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("rows_stg"),
        size: rows_size.max(4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let valid_stg = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("valid_stg"),
        size: valid_size.max(4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let hash_stg = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("hash_stg"),
        size: hash_size.max(4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("enum_bg"),
        layout: &ctx.bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: cfg_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: radixes_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: so_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: sv_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 4, resource: steps_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 5, resource: code_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 6, resource: consts_buf_b.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 7, resource: cr_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 8, resource: dd_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 9, resource: rows_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 10, resource: valid_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 11, resource: hash_buf.as_entire_binding() },
        ],
    });

    let mut encoder = ctx.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&ctx.pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        let groups = total.div_ceil(256);
        pass.dispatch_workgroups(groups, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&rows_buf, 0, &rows_stg, 0, rows_size.max(4));
    encoder.copy_buffer_to_buffer(&valid_buf, 0, &valid_stg, 0, valid_size.max(4));
    encoder.copy_buffer_to_buffer(&hash_buf, 0, &hash_stg, 0, hash_size.max(4));
    ctx.queue.submit(Some(encoder.finish()));

    rows_stg.slice(..).map_async(wgpu::MapMode::Read, |_| {});
    valid_stg.slice(..).map_async(wgpu::MapMode::Read, |_| {});
    hash_stg.slice(..).map_async(wgpu::MapMode::Read, |_| {});
    ctx.device.poll(wgpu::Maintain::Wait);

    let rows_data: Vec<i32> = bytemuck::cast_slice(&rows_stg.slice(..).get_mapped_range()).to_vec();
    let valid_data: Vec<u32> = bytemuck::cast_slice(&valid_stg.slice(..).get_mapped_range()).to_vec();
    let hash_data: Vec<u32> = bytemuck::cast_slice(&hash_stg.slice(..).get_mapped_range()).to_vec();

    // Compact + dedup on CPU
    let mut seen: HashSet<u32> = HashSet::new();
    let mut out: Vec<SurvivorRow> = Vec::new();
    let mut total_kept = 0usize;
    let nv = n_vars as usize;
    for i in 0..(total as usize) {
        if valid_data[i] != 1 {
            continue;
        }
        total_kept += 1;
        if !seen.insert(hash_data[i]) {
            continue;
        }
        let base = i * nv;
        out.push(rows_data[base..base + nv].to_vec());
        if out.len() >= target {
            break;
        }
    }
    (out, total_kept)
}
