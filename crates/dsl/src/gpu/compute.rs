//! wgpu compute pipeline for GPU-accelerated sampling

use std::collections::BTreeMap;

use crate::error::DslError;
use crate::sampler;
use crate::resolver::VarMap;

/// Represents a variable definition compiled for GPU execution
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuSamplerDef {
    kind: u32,
    lo: i32,
    hi: i32,
    dep_a: u32,
    dep_b: u32,
    extra: i32,
    _pad: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuConstraintDef {
    kind: u32,
    var_a: u32,
    var_b: u32,
    constant: i32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuConfig {
    num_vars: u32,
    num_constraints: u32,
    num_invocations: u32,
    seed: u32,
}

/// GPU sampler — generates variable sets in parallel on GPU
pub struct GpuSampler {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl GpuSampler {
    /// Initialize GPU device and compile shader
    pub fn new() -> Result<Self, DslError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }))
        .ok_or_else(|| DslError::DiagramError("No GPU adapter found".into()))?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("locus-dsl-gpu"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            },
            None,
        ))
        .map_err(|e| DslError::DiagramError(format!("GPU device error: {e}")))?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sampler"),
            source: wgpu::ShaderSource::Wgsl(include_str!("sampler.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sampler_bind_group_layout"),
            entries: &[
                // config uniform
                bgl_entry(0, wgpu::BufferBindingType::Uniform),
                // samplers storage
                bgl_entry(1, wgpu::BufferBindingType::Storage { read_only: true }),
                // constraints storage
                bgl_entry(2, wgpu::BufferBindingType::Storage { read_only: true }),
                // choices storage
                bgl_entry(3, wgpu::BufferBindingType::Storage { read_only: true }),
                // results storage (read_write)
                bgl_entry(4, wgpu::BufferBindingType::Storage { read_only: false }),
                // valid storage (read_write)
                bgl_entry(5, wgpu::BufferBindingType::Storage { read_only: false }),
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sampler_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("sampler_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            pipeline,
            bind_group_layout,
        })
    }

    /// Generate `count` variable sets on GPU, return only those passing constraints.
    ///
    /// `variables`: ordered map of variable definitions (same as ProblemSpec.variables)
    /// `constraints`: constraint strings (same as ProblemSpec.constraints)
    /// `count`: number of sets to generate (e.g. 3_000_000)
    ///
    /// Returns vec of VarMaps for passing sets.
    pub fn sample_batch(
        &self,
        variables: &BTreeMap<String, String>,
        constraints: &[String],
        count: u32,
    ) -> Result<Vec<VarMap>, DslError> {
        let var_names: Vec<&String> = variables.keys().collect();
        let num_vars = var_names.len() as u32;

        // Compile variable definitions to GPU format
        let (gpu_samplers, choices_data) = compile_samplers(variables, &var_names)?;
        let gpu_constraints = compile_constraints(constraints, &var_names)?;

        let config = GpuConfig {
            num_vars,
            num_constraints: gpu_constraints.len() as u32,
            num_invocations: count,
            seed: rand::random::<u32>(),
        };

        // Create buffers
        let config_buf = self.create_buffer_init("config", bytemuck::bytes_of(&config), wgpu::BufferUsages::UNIFORM);
        let samplers_buf = self.create_buffer_init("samplers", bytemuck::cast_slice(&gpu_samplers), wgpu::BufferUsages::STORAGE);

        let constraints_data = if gpu_constraints.is_empty() {
            vec![GpuConstraintDef { kind: 0, var_a: 0, var_b: 0, constant: 0 }]
        } else {
            gpu_constraints
        };
        let constraints_buf = self.create_buffer_init("constraints", bytemuck::cast_slice(&constraints_data), wgpu::BufferUsages::STORAGE);

        let choices_padded = if choices_data.is_empty() { vec![0i32] } else { choices_data };
        let choices_buf = self.create_buffer_init("choices", bytemuck::cast_slice(&choices_padded), wgpu::BufferUsages::STORAGE);

        let results_size = (count * num_vars) as usize * std::mem::size_of::<i32>();
        let results_buf = self.create_buffer("results", results_size, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC);

        let valid_size = count as usize * std::mem::size_of::<u32>();
        let valid_buf = self.create_buffer("valid", valid_size, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC);

        // Staging buffers for readback
        let results_staging = self.create_buffer("results_staging", results_size, wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST);
        let valid_staging = self.create_buffer("valid_staging", valid_size, wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST);

        // Bind group
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sampler_bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: config_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: samplers_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: constraints_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: choices_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 4, resource: results_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 5, resource: valid_buf.as_entire_binding() },
            ],
        });

        // Dispatch compute
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups((count + 255) / 256, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&results_buf, 0, &results_staging, 0, results_size as u64);
        encoder.copy_buffer_to_buffer(&valid_buf, 0, &valid_staging, 0, valid_size as u64);
        self.queue.submit(Some(encoder.finish()));

        // Read back results
        let results_slice = results_staging.slice(..);
        let valid_slice = valid_staging.slice(..);
        results_slice.map_async(wgpu::MapMode::Read, |_| {});
        valid_slice.map_async(wgpu::MapMode::Read, |_| {});
        self.device.poll(wgpu::Maintain::Wait);

        let results_data: Vec<i32> = bytemuck::cast_slice(&results_slice.get_mapped_range()).to_vec();
        let valid_data: Vec<u32> = bytemuck::cast_slice(&valid_slice.get_mapped_range()).to_vec();

        // Collect passing sets
        let mut output = Vec::new();
        for i in 0..count as usize {
            if valid_data[i] == 1 {
                let mut vars = VarMap::new();
                for (j, name) in var_names.iter().enumerate() {
                    let val = results_data[i * num_vars as usize + j];
                    vars.insert((*name).clone(), val.to_string());
                }
                output.push(vars);
            }
        }

        Ok(output)
    }

    fn create_buffer(&self, label: &str, size: usize, usage: wgpu::BufferUsages) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: size as u64,
            usage,
            mapped_at_creation: false,
        })
    }

    fn create_buffer_init(&self, label: &str, data: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
        wgpu::util::DeviceExt::create_buffer_init(&self.device, &wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage,
        })
    }
}

fn bgl_entry(binding: u32, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

/// Compile variable definitions to GPU-compatible format
fn compile_samplers(
    variables: &BTreeMap<String, String>,
    var_names: &[&String],
) -> Result<(Vec<GpuSamplerDef>, Vec<i32>), DslError> {
    let mut gpu_samplers = Vec::new();
    let mut choices_data: Vec<i32> = Vec::new();

    let name_to_idx = |name: &str| -> Option<u32> {
        var_names.iter().position(|n| n.as_str() == name).map(|i| i as u32)
    };

    for (name, def) in variables {
        let d = def.trim();
        let gpu = if d.starts_with("integer(") && d.ends_with(')') {
            let args = parse_int_args(d)?;
            GpuSamplerDef { kind: 0, lo: args.0, hi: args.1, dep_a: 0, dep_b: 0, extra: 0, _pad: [0; 2] }
        } else if d.starts_with("nonzero(") && d.ends_with(')') {
            let args = parse_int_args(d)?;
            GpuSamplerDef { kind: 1, lo: args.0, hi: args.1, dep_a: 0, dep_b: 0, extra: 0, _pad: [0; 2] }
        } else if d.starts_with("choice(") && d.ends_with(')') {
            let inner = &d[7..d.len()-1];
            let vals: Vec<i32> = inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            let offset = choices_data.len() as i32;
            let count = vals.len() as i32;
            choices_data.extend(&vals);
            GpuSamplerDef { kind: 2, lo: offset, hi: count - 1, dep_a: 0, dep_b: 0, extra: 0, _pad: [0; 2] }
        } else if let Some((op, a, b)) = parse_binary_op(d, var_names) {
            let kind = match op {
                '+' => 3,
                '*' => 4,
                '-' => 5,
                _ => return Err(DslError::DiagramError(format!("GPU: unsupported op '{op}' in {d}"))),
            };
            GpuSamplerDef { kind, lo: 0, hi: 0, dep_a: a, dep_b: b, extra: 0, _pad: [0; 2] }
        } else if let Some((var_idx, constant)) = parse_scalar_mul(d, var_names) {
            GpuSamplerDef { kind: 6, lo: 0, hi: 0, dep_a: var_idx, dep_b: 0, extra: constant, _pad: [0; 2] }
        } else if let Some(dep_idx) = name_to_idx(d) {
            // Direct copy
            GpuSamplerDef { kind: 7, lo: 0, hi: 0, dep_a: dep_idx, dep_b: 0, extra: 0, _pad: [0; 2] }
        } else {
            return Err(DslError::DiagramError(format!(
                "GPU: can't compile variable '{name}: {d}' — not a simple sampler or arithmetic"
            )));
        };

        gpu_samplers.push(gpu);
    }

    Ok((gpu_samplers, choices_data))
}

/// Compile constraints to GPU format
fn compile_constraints(
    constraints: &[String],
    var_names: &[&String],
) -> Result<Vec<GpuConstraintDef>, DslError> {
    let mut result = Vec::new();

    let name_to_idx = |name: &str| -> Option<u32> {
        var_names.iter().position(|n| n.as_str() == name).map(|i| i as u32)
    };

    for c in constraints {
        let c = c.trim();
        for (op_str, kind) in &[("!=", 0u32), ("<", 1), (">", 2), (">=", 3), ("<=", 4)] {
            if let Some(pos) = c.find(op_str) {
                let lhs = c[..pos].trim();
                let rhs = c[pos + op_str.len()..].trim();

                let var_a = name_to_idx(lhs)
                    .ok_or_else(|| DslError::DiagramError(format!("GPU: unknown var '{lhs}' in constraint")))?;

                let (var_b, constant) = if let Some(idx) = name_to_idx(rhs) {
                    (idx, 0)
                } else if let Ok(val) = rhs.parse::<i32>() {
                    (0xFFFFFFFF, val)
                } else {
                    return Err(DslError::DiagramError(format!("GPU: can't parse constraint RHS '{rhs}'")));
                };

                result.push(GpuConstraintDef { kind: *kind, var_a, var_b, constant });
                break;
            }
        }
    }

    Ok(result)
}

fn parse_int_args(s: &str) -> Result<(i32, i32), DslError> {
    let paren = s.find('(').unwrap();
    let inner = &s[paren+1..s.len()-1];
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 2 {
        return Err(DslError::DiagramError(format!("GPU: expected 2 args in {s}")));
    }
    let lo: i32 = parts[0].trim().parse().map_err(|_| DslError::DiagramError(format!("GPU: bad int {}", parts[0])))?;
    let hi: i32 = parts[1].trim().parse().map_err(|_| DslError::DiagramError(format!("GPU: bad int {}", parts[1])))?;
    Ok((lo, hi))
}

/// Try to parse "a + b" or "a * b" where a and b are known variable names
fn parse_binary_op(s: &str, var_names: &[&String]) -> Option<(char, u32, u32)> {
    for op in ['+', '*', '-'] {
        if let Some(pos) = s.find(|c: char| c == op) {
            let lhs = s[..pos].trim();
            let rhs = s[pos+1..].trim();
            let a = var_names.iter().position(|n| n.as_str() == lhs)?;
            let b = var_names.iter().position(|n| n.as_str() == rhs)?;
            return Some((op, a as u32, b as u32));
        }
    }
    None
}

/// Try to parse "a * 3" or "3 * a" where one side is a variable and other is a constant
fn parse_scalar_mul(s: &str, var_names: &[&String]) -> Option<(u32, i32)> {
    if let Some(pos) = s.find('*') {
        let lhs = s[..pos].trim();
        let rhs = s[pos+1..].trim();

        if let Some(idx) = var_names.iter().position(|n| n.as_str() == lhs) {
            if let Ok(val) = rhs.parse::<i32>() {
                return Some((idx as u32, val));
            }
        }
        if let Some(idx) = var_names.iter().position(|n| n.as_str() == rhs) {
            if let Ok(val) = lhs.parse::<i32>() {
                return Some((idx as u32, val));
            }
        }
    }
    None
}
