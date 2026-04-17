//! Scene builder — parameterised scene recalculation.
//!
//! When a student adjusts a slider, the scene definition is patched and the
//! world is rebuilt.  This module provides helpers for that flow.

use locus_physics_common::scene::SceneDefinition;
use crate::world::PhysicsWorld;

/// Rebuild the world from a scene definition with patched parameter values.
///
/// `overrides` is a list of `(key, value)` pairs where key is in
/// `"body_id.property"` format.
pub fn rebuild_with_params(
    scene: &SceneDefinition,
    overrides: &[(&str, f64)],
) -> PhysicsWorld {
    let mut world = PhysicsWorld::from_scene(scene);
    for &(key, value) in overrides {
        world.set_parameter(key, value, scene);
    }
    world
}
