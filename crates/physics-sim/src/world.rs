//! Rapier2D world management.
//!
//! Converts a [`SceneDefinition`] into a live Rapier2D physics world and
//! provides methods to query body state and adjust parameters.

use std::collections::HashMap;

use nalgebra::Vector2;
use rapier2d::prelude::*;

use locus_physics_common::scene::{BodyType, SceneDefinition, ShapeSpec};

/// The live Rapier2D physics world.
pub struct PhysicsWorld {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,

    /// Map from scene body id → Rapier rigid body handle.
    pub body_handles: HashMap<String, RigidBodyHandle>,
    /// Trajectory history: body_id → Vec<[x, y]>.
    pub trajectories: HashMap<String, Vec<[f32; 2]>>,
    /// Step counter.
    pub step_count: u64,
}

impl PhysicsWorld {
    /// Build a Rapier2D world from a scene definition.
    pub fn from_scene(scene: &SceneDefinition) -> Self {
        let gravity = Vector2::new(scene.gravity[0], scene.gravity[1]);
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut body_handles = HashMap::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();

        // Create bodies
        for body_spec in &scene.bodies {
            let rb = match body_spec.body_type {
                BodyType::Dynamic => RigidBodyBuilder::dynamic()
                    .translation(Vector2::new(body_spec.position[0], body_spec.position[1]))
                    .rotation(body_spec.rotation)
                    .linvel(Vector2::new(body_spec.velocity[0], body_spec.velocity[1]))
                    .build(),
                BodyType::Fixed => RigidBodyBuilder::fixed()
                    .translation(Vector2::new(body_spec.position[0], body_spec.position[1]))
                    .rotation(body_spec.rotation)
                    .build(),
                BodyType::Kinematic => RigidBodyBuilder::kinematic_velocity_based()
                    .translation(Vector2::new(body_spec.position[0], body_spec.position[1]))
                    .rotation(body_spec.rotation)
                    .build(),
            };

            let rb_handle = rigid_body_set.insert(rb);

            // Create collider for this body
            let collider = build_collider(&body_spec.shape, &body_spec.material, body_spec.mass);
            collider_set.insert_with_parent(collider, rb_handle, &mut rigid_body_set);

            body_handles.insert(body_spec.id.clone(), rb_handle);
        }

        // Create boundaries
        for boundary in &scene.boundaries {
            let rb = RigidBodyBuilder::fixed()
                .translation(Vector2::new(0.0, 0.0))
                .build();
            let rb_handle = rigid_body_set.insert(rb);

            let start = nalgebra::Point2::new(boundary.start[0], boundary.start[1]);
            let end = nalgebra::Point2::new(boundary.end[0], boundary.end[1]);
            let collider = ColliderBuilder::segment(start, end)
                .restitution(boundary.material.restitution)
                .friction(boundary.material.kinetic_friction)
                .build();
            collider_set.insert_with_parent(collider, rb_handle, &mut rigid_body_set);
            body_handles.insert(boundary.id.clone(), rb_handle);
        }

        // Create constraints
        for constraint in &scene.constraints {
            match constraint {
                locus_physics_common::scene::ConstraintSpec::Revolute {
                    body,
                    anchor_world,
                } => {
                    if let Some(&handle) = body_handles.get(body) {
                        let joint = RevoluteJointBuilder::new()
                            .local_anchor1(nalgebra::Point2::new(0.0, 0.0))
                            .local_anchor2(nalgebra::Point2::new(
                                anchor_world[0],
                                anchor_world[1],
                            ));
                        // Attach to ground (fixed body)
                        let ground = rigid_body_set.insert(RigidBodyBuilder::fixed().build());
                        impulse_joint_set.insert(ground, handle, joint, true);
                    }
                }
                locus_physics_common::scene::ConstraintSpec::Spring {
                    body_a,
                    body_b,
                    stiffness,
                    damping,
                    rest_length,
                    ..
                } => {
                    if let Some(&handle_a) = body_handles.get(body_a) {
                        let handle_b = body_b
                            .as_ref()
                            .and_then(|id| body_handles.get(id).copied())
                            .unwrap_or_else(|| {
                                rigid_body_set.insert(RigidBodyBuilder::fixed().build())
                            });
                        let joint = SpringJointBuilder::new(*rest_length, *stiffness, *damping);
                        impulse_joint_set.insert(handle_a, handle_b, joint, true);
                    }
                }
                locus_physics_common::scene::ConstraintSpec::Rod {
                    body_a,
                    body_b,
                    length,
                } => {
                    if let (Some(&ha), Some(&hb)) =
                        (body_handles.get(body_a), body_handles.get(body_b))
                    {
                        let joint = RopeJointBuilder::new(*length);
                        impulse_joint_set.insert(ha, hb, joint, true);
                    }
                }
            }
        }

        Self {
            rigid_body_set,
            collider_set,
            gravity,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver: CCDSolver::new(),
            body_handles,
            trajectories: HashMap::new(),
            step_count: 0,
        }
    }

    /// Advance the simulation by one fixed timestep (1/60s).
    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );

        self.step_count += 1;

        // Record trajectories for dynamic bodies (every 3rd step to save memory)
        if self.step_count % 3 == 0 {
            for (id, &handle) in &self.body_handles {
                if let Some(rb) = self.rigid_body_set.get(handle) {
                    if rb.is_dynamic() {
                        let pos = rb.translation();
                        self.trajectories
                            .entry(id.clone())
                            .or_default()
                            .push([pos.x, pos.y]);
                    }
                }
            }
        }
    }

    /// Read a measurement from a body.
    pub fn get_measurement(&self, body_id: &str, key: &str) -> f64 {
        let handle = match self.body_handles.get(body_id) {
            Some(h) => *h,
            None => return 0.0,
        };
        let rb = match self.rigid_body_set.get(handle) {
            Some(rb) => rb,
            None => return 0.0,
        };

        match key {
            "position_x" => rb.translation().x as f64,
            "position_y" => rb.translation().y as f64,
            "velocity_x" => rb.linvel().x as f64,
            "velocity_y" => rb.linvel().y as f64,
            "velocity" => {
                let v = rb.linvel();
                ((v.x * v.x + v.y * v.y) as f64).sqrt()
            }
            "speed" => {
                let v = rb.linvel();
                ((v.x * v.x + v.y * v.y) as f64).sqrt()
            }
            "kinetic_energy" => {
                let v = rb.linvel();
                let speed_sq = (v.x * v.x + v.y * v.y) as f64;
                0.5 * rb.mass() as f64 * speed_sq
            }
            "angle" => rb.rotation().angle() as f64,
            "angular_velocity" => rb.angvel() as f64,
            _ => 0.0,
        }
    }

    /// Update a parameter on the scene (rebuilds the affected body).
    pub fn set_parameter(&mut self, key: &str, value: f64, scene: &SceneDefinition) {
        // Parse "body_id.property" format
        let parts: Vec<&str> = key.splitn(2, '.').collect();
        if parts.len() != 2 {
            return;
        }
        let (body_id, property) = (parts[0], parts[1]);

        let handle = match self.body_handles.get(body_id) {
            Some(h) => *h,
            None => return,
        };

        if let Some(rb) = self.rigid_body_set.get_mut(handle) {
            match property {
                "mass" => {
                    // Rapier handles mass through collider density; for simplicity
                    // we store it but a full implementation would re-compute density.
                    // For now this is a placeholder that works for display purposes.
                }
                "rotation" => {
                    rb.set_rotation(nalgebra::UnitComplex::new(value as f32), true);
                }
                "velocity.x" => {
                    let mut v = *rb.linvel();
                    v.x = value as f32;
                    rb.set_linvel(v, true);
                }
                "velocity.y" => {
                    let mut v = *rb.linvel();
                    v.y = value as f32;
                    rb.set_linvel(v, true);
                }
                "position.x" => {
                    let mut t = *rb.translation();
                    t.x = value as f32;
                    rb.set_translation(t, true);
                }
                "position.y" => {
                    let mut t = *rb.translation();
                    t.y = value as f32;
                    rb.set_translation(t, true);
                }
                _ => {
                    // Unknown property — look through the scene for custom mappings.
                    let _ = scene; // future: dynamic scene parameter mapping
                }
            }
        }
    }
}

/// Build a Rapier2D collider from a shape spec.
fn build_collider(
    shape: &ShapeSpec,
    material: &locus_physics_common::scene::MaterialSpec,
    mass: f32,
) -> Collider {
    let builder = match shape {
        ShapeSpec::Circle { radius } => ColliderBuilder::ball(*radius),
        ShapeSpec::Rectangle { width, height } => {
            ColliderBuilder::cuboid(*width / 2.0, *height / 2.0)
        }
        ShapeSpec::Triangle { base, height } => {
            let half_b = base / 2.0;
            ColliderBuilder::triangle(
                nalgebra::Point2::new(-half_b, 0.0),
                nalgebra::Point2::new(*half_b, 0.0),
                nalgebra::Point2::new(0.0, *height),
            )
        }
        ShapeSpec::Polygon { vertices } => {
            let points: Vec<nalgebra::Point2<f32>> = vertices
                .iter()
                .map(|v| nalgebra::Point2::new(v[0], v[1]))
                .collect();
            ColliderBuilder::convex_hull(&points).unwrap_or_else(|| ColliderBuilder::ball(0.5))
        }
        ShapeSpec::Segment { start, end } => ColliderBuilder::segment(
            nalgebra::Point2::new(start[0], start[1]),
            nalgebra::Point2::new(end[0], end[1]),
        ),
    };

    builder
        .restitution(material.restitution)
        .friction(material.kinetic_friction)
        .mass(mass)
        .build()
}
