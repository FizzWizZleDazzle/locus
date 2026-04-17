//! Scene definition types.
//!
//! A [`SceneDefinition`] fully describes the initial state of a Rapier2D
//! simulation in a serialisable, physics-engine-agnostic format.  The
//! frontend's `physics-sim` crate converts this into a live Rapier2D world.

use serde::{Deserialize, Serialize};

// ============================================================================
// Top-level scene
// ============================================================================

/// Everything the simulation engine needs to construct a world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDefinition {
    /// Gravity vector `[x, y]` in m/s^2.  Typically `[0.0, -9.81]`.
    pub gravity: [f32; 2],
    /// Rigid bodies that make up the scene.
    pub bodies: Vec<BodySpec>,
    /// Optional constraints / joints between bodies.
    #[serde(default)]
    pub constraints: Vec<ConstraintSpec>,
    /// World boundaries (floor, walls, ceiling).
    #[serde(default)]
    pub boundaries: Vec<BoundarySpec>,
    /// Camera defaults: `[center_x, center_y, zoom]`.
    #[serde(default = "default_camera")]
    pub camera: [f32; 3],
    /// Pixels-per-metre scale for rendering.
    #[serde(default = "default_ppm")]
    pub pixels_per_metre: f32,
}

fn default_camera() -> [f32; 3] {
    [0.0, 0.0, 1.0]
}

fn default_ppm() -> f32 {
    50.0
}

// ============================================================================
// Bodies
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySpec {
    /// Unique label used to reference this body in challenge stages / overlays.
    pub id: String,
    /// Human-readable label shown on the canvas (e.g. "Block A").
    #[serde(default)]
    pub label: String,
    pub body_type: BodyType,
    pub shape: ShapeSpec,
    /// Position `[x, y]` in world metres.
    pub position: [f32; 2],
    /// Rotation in radians.
    #[serde(default)]
    pub rotation: f32,
    /// Initial velocity `[vx, vy]` in m/s.
    #[serde(default)]
    pub velocity: [f32; 2],
    pub material: MaterialSpec,
    /// Mass in kg (only meaningful for Dynamic bodies).
    #[serde(default = "default_mass")]
    pub mass: f32,
    /// Fill colour as a CSS colour string.
    #[serde(default = "default_fill")]
    pub fill_color: String,
    /// Stroke colour.
    #[serde(default = "default_stroke")]
    pub stroke_color: String,
}

fn default_mass() -> f32 {
    1.0
}

fn default_fill() -> String {
    "#93c5fd".into() // blue-300
}

fn default_stroke() -> String {
    "#1e3a5f".into()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BodyType {
    /// Fully simulated body.
    Dynamic,
    /// Immovable (ramps, floors, walls).
    Fixed,
    /// Moved by the user / script, not by forces.
    Kinematic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ShapeSpec {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Triangle { base: f32, height: f32 },
    Polygon { vertices: Vec<[f32; 2]> },
    /// A thin segment, used for ramps / inclined planes.
    Segment { start: [f32; 2], end: [f32; 2] },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialSpec {
    /// Coefficient of restitution (bounciness) `[0, 1]`.
    #[serde(default)]
    pub restitution: f32,
    /// Static friction coefficient.
    #[serde(default)]
    pub static_friction: f32,
    /// Kinetic (dynamic) friction coefficient.
    #[serde(default)]
    pub kinetic_friction: f32,
}

// ============================================================================
// Constraints
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConstraintSpec {
    /// Spring connecting two bodies (or a body to a fixed anchor).
    Spring {
        body_a: String,
        body_b: Option<String>,
        anchor_a: [f32; 2],
        anchor_b: [f32; 2],
        rest_length: f32,
        stiffness: f32,
        damping: f32,
    },
    /// Fixed pivot (pendulum).
    Revolute {
        body: String,
        anchor_world: [f32; 2],
    },
    /// Rod of fixed length between two bodies.
    Rod {
        body_a: String,
        body_b: String,
        length: f32,
    },
}

// ============================================================================
// Boundaries
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundarySpec {
    pub id: String,
    /// Start point in world metres.
    pub start: [f32; 2],
    /// End point in world metres.
    pub end: [f32; 2],
    pub material: MaterialSpec,
}

// ============================================================================
// Adjustable parameters
// ============================================================================

/// A single adjustable slider exposed to the student.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterSpec {
    /// Key used in `set_parameter` calls (e.g. "mass", "angle").
    pub key: String,
    /// Human-readable label shown next to the slider.
    pub label: String,
    pub default: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    /// Unit label shown after the value (e.g. "kg", "deg", "m/s").
    #[serde(default)]
    pub unit: String,
    /// Which body / property this parameter maps to.  The simulation engine
    /// interprets this to know *what* to update when the slider changes.
    ///
    /// Format: `"body_id.property"` e.g. `"block.mass"`, `"ramp.rotation"`,
    /// `"ball.velocity.x"`.
    pub target: String,
}
