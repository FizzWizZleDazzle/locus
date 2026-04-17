//! Canvas2D rendering for physics bodies.
//!
//! Draws a clean, diagrammatic style: pastel fills, thin outlines, grid
//! background.  All coordinates go through the scene's pixels-per-metre scale.

use web_sys::CanvasRenderingContext2d;

use locus_physics_common::scene::{BodyType, SceneDefinition, ShapeSpec};

use crate::world::PhysicsWorld;

/// Background colour and grid.
pub fn draw_background(ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
    // Light background
    ctx.set_fill_style_str("#f8fafc");
    ctx.fill_rect(0.0, 0.0, w, h);

    // Subtle grid
    ctx.set_stroke_style_str("rgba(0,0,0,0.06)");
    ctx.set_line_width(0.5);

    let grid = 40.0;
    let mut x = 0.0;
    while x <= w {
        ctx.begin_path();
        ctx.move_to(x, 0.0);
        ctx.line_to(x, h);
        ctx.stroke();
        x += grid;
    }
    let mut y = 0.0;
    while y <= h {
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(w, y);
        ctx.stroke();
        y += grid;
    }
}

/// Draw all rigid bodies from the scene.
pub fn draw_bodies(
    ctx: &CanvasRenderingContext2d,
    world: &PhysicsWorld,
    scene: &SceneDefinition,
) {
    let ppm = scene.pixels_per_metre as f64;
    let canvas_w = ctx.canvas().map(|c| c.width() as f64).unwrap_or(800.0);
    let canvas_h = ctx.canvas().map(|c| c.height() as f64).unwrap_or(600.0);

    // Coordinate transform: world (0,0) at center-bottom of canvas,
    // y-up in physics → y-down on canvas.
    let origin_x = canvas_w / 2.0 + scene.camera[0] as f64 * ppm;
    let origin_y = canvas_h * 0.75 - scene.camera[1] as f64 * ppm;

    for body_spec in &scene.bodies {
        let handle = match world.body_handles.get(&body_spec.id) {
            Some(h) => *h,
            None => continue,
        };
        let rb = match world.rigid_body_set.get(handle) {
            Some(rb) => rb,
            None => continue,
        };

        let pos = rb.translation();
        let angle = rb.rotation().angle() as f64;

        // World → canvas coordinates
        let cx = origin_x + pos.x as f64 * ppm;
        let cy = origin_y - pos.y as f64 * ppm;

        ctx.save();
        ctx.translate(cx, cy).ok();
        ctx.rotate(-angle).ok(); // negate because canvas y is flipped

        // Set colours
        ctx.set_fill_style_str(&body_spec.fill_color);
        ctx.set_stroke_style_str(&body_spec.stroke_color);
        ctx.set_line_width(if body_spec.body_type == BodyType::Fixed {
            2.0
        } else {
            1.5
        });

        match &body_spec.shape {
            ShapeSpec::Circle { radius } => {
                let r = *radius as f64 * ppm;
                ctx.begin_path();
                ctx.arc(0.0, 0.0, r, 0.0, std::f64::consts::TAU).ok();
                ctx.fill();
                ctx.stroke();
            }
            ShapeSpec::Rectangle { width, height } => {
                let w = *width as f64 * ppm;
                let h = *height as f64 * ppm;
                ctx.begin_path();
                ctx.rect(-w / 2.0, -h / 2.0, w, h);
                ctx.fill();
                ctx.stroke();
            }
            ShapeSpec::Triangle { base, height } => {
                let b = *base as f64 * ppm;
                let h = *height as f64 * ppm;
                ctx.begin_path();
                ctx.move_to(-b / 2.0, 0.0);
                ctx.line_to(b / 2.0, 0.0);
                ctx.line_to(0.0, -h); // up in canvas coords
                ctx.close_path();
                ctx.fill();
                ctx.stroke();
            }
            ShapeSpec::Polygon { vertices } => {
                if vertices.len() >= 2 {
                    ctx.begin_path();
                    ctx.move_to(
                        vertices[0][0] as f64 * ppm,
                        -vertices[0][1] as f64 * ppm,
                    );
                    for v in &vertices[1..] {
                        ctx.line_to(v[0] as f64 * ppm, -v[1] as f64 * ppm);
                    }
                    ctx.close_path();
                    ctx.fill();
                    ctx.stroke();
                }
            }
            ShapeSpec::Segment { start, end } => {
                ctx.begin_path();
                ctx.move_to(start[0] as f64 * ppm, -start[1] as f64 * ppm);
                ctx.line_to(end[0] as f64 * ppm, -end[1] as f64 * ppm);
                ctx.set_line_width(3.0);
                ctx.stroke();
            }
        }

        // Body label
        if !body_spec.label.is_empty() {
            ctx.set_fill_style_str("#1e293b");
            ctx.set_font("bold 12px system-ui, sans-serif");
            ctx.set_text_align("center");
            ctx.fill_text(&body_spec.label, 0.0, -4.0).ok();
        }

        ctx.restore();
    }
}

/// Semi-transparent overlay shown when the simulation is locked.
pub fn draw_locked_overlay(ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
    ctx.set_fill_style_str("rgba(0, 0, 0, 0.05)");
    ctx.fill_rect(0.0, 0.0, w, h);

    ctx.set_fill_style_str("#64748b");
    ctx.set_font("600 14px system-ui, sans-serif");
    ctx.set_text_align("center");
    ctx.fill_text("Complete the prediction to unlock the simulation", w / 2.0, 30.0)
        .ok();
}
