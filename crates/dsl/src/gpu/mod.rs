//! Combinatorial enumeration with bytecode VM.
//!
//! - `bytecode` — opcode set, stack-machine VM, encoder.
//! - `compile`  — turn ProblemSpec variables/constraints into an executable `Plan`.
//! - `cpu_exec` — rayon-parallel Cartesian-product enumeration on CPU.
//! - `enumerator` — top-level driver: compile → execute → render.
//!
//! GPU executor (wgpu) lands in M2.

pub mod bytecode;
pub mod compile;
pub mod cpu_exec;
pub mod enumerator;
pub mod hoist;

#[cfg(feature = "gpu")]
pub mod gpu_exec;

pub use enumerator::{Executor, enumerate};
