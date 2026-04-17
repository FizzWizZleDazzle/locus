//! Simulation state checkpoints — save / restore for solution step navigation.

use crate::world::PhysicsWorld;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Checkpoint {
    bodies: HashMap<String, BodyCheckpoint>,
    step_count: u64,
}

#[derive(Serialize, Deserialize)]
struct BodyCheckpoint {
    position: [f32; 2],
    rotation: f32,
    linvel: [f32; 2],
    angvel: f32,
}

/// Serialise the current world state to JSON.
pub fn save(world: &PhysicsWorld) -> String {
    let mut bodies = HashMap::new();
    for (id, &handle) in &world.body_handles {
        if let Some(rb) = world.rigid_body_set.get(handle) {
            let t = rb.translation();
            let v = rb.linvel();
            bodies.insert(
                id.clone(),
                BodyCheckpoint {
                    position: [t.x, t.y],
                    rotation: rb.rotation().angle(),
                    linvel: [v.x, v.y],
                    angvel: rb.angvel(),
                },
            );
        }
    }

    let cp = Checkpoint {
        bodies,
        step_count: world.step_count,
    };
    serde_json::to_string(&cp).unwrap_or_default()
}

/// Restore a previously saved checkpoint.
pub fn load(world: &mut PhysicsWorld, json: &str) -> Result<(), String> {
    let cp: Checkpoint =
        serde_json::from_str(json).map_err(|e| format!("Invalid checkpoint: {}", e))?;

    for (id, state) in &cp.bodies {
        if let Some(&handle) = world.body_handles.get(id) {
            if let Some(rb) = world.rigid_body_set.get_mut(handle) {
                rb.set_translation(
                    nalgebra::Vector2::new(state.position[0], state.position[1]),
                    true,
                );
                rb.set_rotation(nalgebra::UnitComplex::new(state.rotation), true);
                rb.set_linvel(
                    nalgebra::Vector2::new(state.linvel[0], state.linvel[1]),
                    true,
                );
                rb.set_angvel(state.angvel, true);
            }
        }
    }

    world.step_count = cp.step_count;
    Ok(())
}
