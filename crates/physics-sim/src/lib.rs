//! Physics simulation engine for the Locus learning platform.
//!
//! This crate compiles to a **standalone WASM module** (cdylib) that is
//! lazy-loaded when the user navigates to `/physics`.  It wraps Rapier2D for
//! physics, Canvas2D for rendering, and exposes a `wasm_bindgen` API that the
//! Leptos frontend calls.

mod checkpoint;
mod controller;
mod overlays;
mod renderer;
mod scene_builder;
mod world;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use locus_physics_common::scene::SceneDefinition;

pub use controller::SimState;

/// The main simulation engine exposed to JavaScript / Leptos.
#[wasm_bindgen]
pub struct SimulationEngine {
    /// Rapier2D world state.
    world: world::PhysicsWorld,
    /// Canvas 2D rendering context.
    ctx: CanvasRenderingContext2d,
    /// The canvas element (for dimension queries).
    canvas: HtmlCanvasElement,
    /// Simulation controller (play/pause/speed).
    controller: controller::Controller,
    /// The original scene definition (for reset).
    scene_def: SceneDefinition,
    /// Overlay visibility flags.
    overlays: overlays::OverlayState,
    /// Whether the simulation is locked (student hasn't predicted yet).
    locked: bool,
}

#[wasm_bindgen]
impl SimulationEngine {
    /// Create a new simulation from a JSON scene definition.
    ///
    /// `canvas_id` is the DOM id of the `<canvas>` element.
    /// `scene_json` is the serialised `SceneDefinition`.
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, scene_json: &str) -> Result<SimulationEngine, JsValue> {
        console_error_panic_hook::set_once();

        let scene_def: SceneDefinition = serde_json::from_str(scene_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid scene JSON: {}", e)))?;

        let document = web_sys::window()
            .ok_or("no window")?
            .document()
            .ok_or("no document")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| JsValue::from_str(&format!("canvas '{}' not found", canvas_id)))?
            .dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas
            .get_context("2d")?
            .ok_or("failed to get 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let physics_world = world::PhysicsWorld::from_scene(&scene_def);
        let controller = controller::Controller::new();
        let overlay_state = overlays::OverlayState::default();

        Ok(Self {
            world: physics_world,
            ctx,
            canvas,
            controller,
            scene_def,
            overlays: overlay_state,
            locked: true, // simulation starts locked
        })
    }

    // ── Playback controls ────────────────────────────────────────────

    /// Unlock the simulation (called after the student commits a prediction).
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    /// Is the simulation currently locked?
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn play(&mut self) {
        if !self.locked {
            self.controller.play();
        }
    }

    pub fn pause(&mut self) {
        self.controller.pause();
    }

    /// Advance the simulation by one fixed timestep.
    pub fn step_forward(&mut self) {
        if !self.locked {
            self.world.step();
        }
    }

    /// Reset the simulation to the initial scene state.
    pub fn reset(&mut self) {
        self.world = world::PhysicsWorld::from_scene(&self.scene_def);
        self.controller.reset();
    }

    /// Set the playback speed multiplier (e.g. 0.25 for quarter speed).
    pub fn set_speed(&mut self, multiplier: f32) {
        self.controller.set_speed(multiplier);
    }

    /// Get the current simulation time in seconds.
    pub fn sim_time(&self) -> f32 {
        self.controller.sim_time()
    }

    pub fn state(&self) -> String {
        format!("{:?}", self.controller.state())
    }

    // ── Parameter adjustment ─────────────────────────────────────────

    /// Update an adjustable parameter (mass, angle, etc.).
    pub fn set_parameter(&mut self, key: &str, value: f64) {
        self.world.set_parameter(key, value, &self.scene_def);
    }

    // ── Overlay toggles ──────────────────────────────────────────────

    pub fn toggle_overlay(&mut self, name: &str, visible: bool) {
        self.overlays.set(name, visible);
    }

    // ── Measurements ─────────────────────────────────────────────────

    /// Read a current measurement from the simulation.
    /// Keys: "velocity", "position_x", "position_y", "kinetic_energy", etc.
    pub fn get_measurement(&self, body_id: &str, key: &str) -> f64 {
        self.world.get_measurement(body_id, key)
    }

    // ── Checkpoints ──────────────────────────────────────────────────

    /// Serialise the current simulation state to JSON.
    pub fn save_checkpoint(&self) -> String {
        checkpoint::save(&self.world)
    }

    /// Restore a previously saved checkpoint.
    pub fn load_checkpoint(&mut self, json: &str) -> Result<(), JsValue> {
        checkpoint::load(&mut self.world, json)
            .map_err(|e| JsValue::from_str(&format!("Checkpoint load failed: {}", e)))
    }

    // ── Render loop entry point ──────────────────────────────────────

    /// Called once per `requestAnimationFrame`.  Steps physics (if playing)
    /// and redraws the canvas.
    pub fn tick(&mut self, dt_ms: f64) {
        let dt = (dt_ms / 1000.0) as f32;

        // Step physics if playing and unlocked
        if self.controller.is_playing() && !self.locked {
            let steps = self.controller.steps_for_dt(dt);
            for _ in 0..steps {
                self.world.step();
            }
            self.controller.advance(steps);
        }

        // Render
        let w = self.canvas.width() as f64;
        let h = self.canvas.height() as f64;
        self.ctx.clear_rect(0.0, 0.0, w, h);

        renderer::draw_background(&self.ctx, w, h);
        renderer::draw_bodies(&self.ctx, &self.world, &self.scene_def);

        if self.overlays.show_forces {
            overlays::draw_forces(&self.ctx, &self.world, &self.scene_def);
        }
        if self.overlays.show_velocity {
            overlays::draw_velocity(&self.ctx, &self.world, &self.scene_def);
        }
        if self.overlays.show_trajectory {
            overlays::draw_trajectory(&self.ctx, &self.world);
        }
        if self.overlays.show_labels {
            overlays::draw_labels(&self.ctx, &self.world, &self.scene_def);
        }

        // Locked overlay
        if self.locked {
            renderer::draw_locked_overlay(&self.ctx, w, h);
        }
    }

    // ── Cleanup ──────────────────────────────────────────────────────

    /// Release resources.  Call this when the component unmounts.
    pub fn destroy(self) {
        // Drop is automatic; this is for explicit JS-side lifecycle.
        drop(self);
    }
}
