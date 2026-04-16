//! GPU-accelerated sampling and constraint checking via wgpu compute shaders.
//!
//! Generates millions of variable sets in parallel on GPU, filters by constraints,
//! returns passing sets to CPU for expression evaluation and LaTeX rendering.

#[cfg(feature = "gpu")]
mod compute;

#[cfg(feature = "gpu")]
pub use compute::GpuSampler;
