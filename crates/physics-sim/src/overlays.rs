//! Visual overlays: force vectors, velocity arrows, trajectory traces, labels.

use web_sys::CanvasRenderingContext2d;

use locus_physics_common::scene::{BodyType, SceneDefinition};

use crate::world::PhysicsWorld;

/// Overlay visibility flags.
#[derive(Debug, Clone)]
pub struct OverlayState {
    pub show_forces: bool,
    pub show_velocity: bool,
    pub show_trajectory: bool,
    pub show_labels: bool,
    pub show_dimensions: bool,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            show_forces: false,
            show_velocity: false,
            show_trajectory: false,
            show_labels: true,
            show_dimensions: false,
        }
    }
}

impl OverlayState {
    pub fn set(&mut self, name: &str, visible: bool) {
        match name {
            "forces" => self.show_forces = visible,
            "velocity" => self.show_velocity = visible,
            "trajectory" => self.show_trajectory = visible,
            "labels" => self.show_labels = visible,
            "dimensions" => self.show_dimensions = visible,
            _ => {}
        }
    }
}

// ============================================================================
// Drawing helpers
// ============================================================================

/// Draw an arrow from `(x1,y1)` to `(x2,y2)` with a given colour.
fn draw_arrow(
    ctx: &CanvasRenderingContext2d,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: &str,
    line_width: f64,
) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 2.0 {
        return;
    }

    let angle = dy.atan2(dx);
    let head_len = (len * 0.25).min(12.0).max(6.0);

    ctx.set_stroke_style_str(color);
    ctx.set_fill_style_str(color);
    ctx.set_line_width(line_width);

    // Shaft
    ctx.begin_path();
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.stroke();

    // Arrowhead
    ctx.begin_path();
    ctx.move_to(x2, y2);
    ctx.line_to(
        x2 - head_len * (angle - 0.4).cos(),
        y2 - head_len * (angle - 0.4).sin(),
    );
    ctx.line_to(
        x2 - head_len * (angle + 0.4).cos(),
        y2 - head_len * (angle + 0.4).sin(),
    );
    ctx.close_path();
    ctx.fill();
}

/// Compute canvas coordinates for a body.
fn body_canvas_pos(
    world: &PhysicsWorld,
    body_id: &str,
    scene: &SceneDefinition,
    canvas_w: f64,
    canvas_h: f64,
) -> Option<(f64, f64)> {
    let ppm = scene.pixels_per_metre as f64;
    let origin_x = canvas_w / 2.0 + scene.camera[0] as f64 * ppm;
    let origin_y = canvas_h * 0.75 - scene.camera[1] as f64 * ppm;

    let handle = world.body_handles.get(body_id)?;
    let rb = world.rigid_body_set.get(*handle)?;
    let pos = rb.translation();
    Some((
        origin_x + pos.x as f64 * ppm,
        origin_y - pos.y as f64 * ppm,
    ))
}

// ============================================================================
// Overlay drawers
// ============================================================================

/// Draw gravity and applied force vectors on dynamic bodies.
pub fn draw_forces(
    ctx: &CanvasRenderingContext2d,
    world: &PhysicsWorld,
    scene: &SceneDefinition,
) {
    let ppm = scene.pixels_per_metre as f64;
    let canvas_w = ctx.canvas().map(|c| c.width() as f64).unwrap_or(800.0);
    let canvas_h = ctx.canvas().map(|c| c.height() as f64).unwrap_or(600.0);
    let scale = 3.0; // force vector visual scale

    for body_spec in &scene.bodies {
        if body_spec.body_type != BodyType::Dynamic {
            continue;
        }

        if let Some((cx, cy)) = body_canvas_pos(world, &body_spec.id, scene, canvas_w, canvas_h) {
            let handle = match world.body_handles.get(&body_spec.id) {
                Some(h) => *h,
                None => continue,
            };
            let rb = match world.rigid_body_set.get(handle) {
                Some(rb) => rb,
                None => continue,
            };

            let mass = rb.mass() as f64;

            // Gravity vector (downward in world → downward on canvas)
            let gy = scene.gravity[1] as f64; // typically -9.81
            let gravity_px = -gy * mass * scale; // positive = down on canvas
            draw_arrow(ctx, cx, cy, cx, cy + gravity_px, "#ef4444", 2.5);

            // Label
            ctx.set_fill_style_str("#ef4444");
            ctx.set_font("bold 11px system-ui, sans-serif");
            ctx.set_text_align("left");
            ctx.fill_text("mg", cx + 6.0, cy + gravity_px / 2.0).ok();
        }
    }
}

/// Draw velocity vectors.
pub fn draw_velocity(
    ctx: &CanvasRenderingContext2d,
    world: &PhysicsWorld,
    scene: &SceneDefinition,
) {
    let ppm = scene.pixels_per_metre as f64;
    let canvas_w = ctx.canvas().map(|c| c.width() as f64).unwrap_or(800.0);
    let canvas_h = ctx.canvas().map(|c| c.height() as f64).unwrap_or(600.0);
    let scale = ppm * 0.3; // velocity visual scale

    for body_spec in &scene.bodies {
        if body_spec.body_type != BodyType::Dynamic {
            continue;
        }

        if let Some((cx, cy)) = body_canvas_pos(world, &body_spec.id, scene, canvas_w, canvas_h) {
            let handle = match world.body_handles.get(&body_spec.id) {
                Some(h) => *h,
                None => continue,
            };
            let rb = match world.rigid_body_set.get(handle) {
                Some(rb) => rb,
                None => continue,
            };

            let vx = rb.linvel().x as f64;
            let vy = rb.linvel().y as f64;

            let end_x = cx + vx * scale;
            let end_y = cy - vy * scale; // flip y

            draw_arrow(ctx, cx, cy, end_x, end_y, "#2563eb", 2.0);
        }
    }
}

/// Draw trajectory traces.
pub fn draw_trajectory(
    ctx: &CanvasRenderingContext2d,
    world: &PhysicsWorld,
) {
    ctx.set_stroke_style_str("rgba(139, 92, 246, 0.5)"); // purple
    ctx.set_line_width(1.5);
    ctx.set_line_dash(&js_sys::Array::of2(
        &wasm_bindgen::JsValue::from_f64(4.0),
        &wasm_bindgen::JsValue::from_f64(4.0),
    ))
    .ok();

    for (_body_id, points) in &world.trajectories {
        if points.len() < 2 {
            continue;
        }
        ctx.begin_path();
        // Note: these are already in world coords; we'd need the transform.
        // For now we store canvas-space points in a future iteration.
        // This is a simplified placeholder.
        for (i, pt) in points.iter().enumerate() {
            if i == 0 {
                ctx.move_to(pt[0] as f64, pt[1] as f64);
            } else {
                ctx.line_to(pt[0] as f64, pt[1] as f64);
            }
        }
        ctx.stroke();
    }

    // Reset dash
    ctx.set_line_dash(&js_sys::Array::new()).ok();
}

/// Draw body mass / dimension labels.
pub fn draw_labels(
    ctx: &CanvasRenderingContext2d,
    world: &PhysicsWorld,
    scene: &SceneDefinition,
) {
    let canvas_w = ctx.canvas().map(|c| c.width() as f64).unwrap_or(800.0);
    let canvas_h = ctx.canvas().map(|c| c.height() as f64).unwrap_or(600.0);

    ctx.set_fill_style_str("#475569");
    ctx.set_font("11px system-ui, sans-serif");
    ctx.set_text_align("center");

    for body_spec in &scene.bodies {
        if body_spec.body_type != BodyType::Dynamic {
            continue;
        }

        if let Some((cx, cy)) = body_canvas_pos(world, &body_spec.id, scene, canvas_w, canvas_h) {
            let handle = match world.body_handles.get(&body_spec.id) {
                Some(h) => *h,
                None => continue,
            };
            let rb = match world.rigid_body_set.get(handle) {
                Some(rb) => rb,
                None => continue,
            };

            let mass = rb.mass();
            let speed = {
                let v = rb.linvel();
                (v.x * v.x + v.y * v.y).sqrt()
            };

            let label = format!("{:.1} kg | {:.1} m/s", mass, speed);
            ctx.fill_text(&label, cx, cy + 20.0).ok();
        }
    }
}
