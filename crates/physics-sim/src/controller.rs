//! Simulation playback controller.
//!
//! Manages play/pause state, speed multiplier, and timestep accounting.

/// Playback state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimState {
    Paused,
    Playing,
}

pub struct Controller {
    state: SimState,
    /// Speed multiplier (1.0 = real-time, 0.25 = quarter speed).
    speed: f32,
    /// Accumulated sim time in seconds.
    sim_time: f32,
    /// Fixed timestep (1/60s).
    dt: f32,
    /// Accumulator for sub-step timing.
    accumulator: f32,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: SimState::Paused,
            speed: 1.0,
            sim_time: 0.0,
            dt: 1.0 / 60.0,
            accumulator: 0.0,
        }
    }

    pub fn play(&mut self) {
        self.state = SimState::Playing;
    }

    pub fn pause(&mut self) {
        self.state = SimState::Paused;
    }

    pub fn reset(&mut self) {
        self.state = SimState::Paused;
        self.sim_time = 0.0;
        self.accumulator = 0.0;
    }

    pub fn set_speed(&mut self, multiplier: f32) {
        self.speed = multiplier.clamp(0.1, 4.0);
    }

    pub fn is_playing(&self) -> bool {
        self.state == SimState::Playing
    }

    pub fn state(&self) -> SimState {
        self.state
    }

    pub fn sim_time(&self) -> f32 {
        self.sim_time
    }

    /// Given the wall-clock delta `dt_wall` (in seconds), compute how many
    /// fixed physics steps should run this frame.
    pub fn steps_for_dt(&mut self, dt_wall: f32) -> u32 {
        self.accumulator += dt_wall * self.speed;
        let steps = (self.accumulator / self.dt) as u32;
        self.accumulator -= steps as f32 * self.dt;
        // Cap to prevent spiral-of-death on lag spikes.
        steps.min(10)
    }

    /// Advance the sim clock by `n` fixed steps.
    pub fn advance(&mut self, steps: u32) {
        self.sim_time += steps as f32 * self.dt;
    }
}
