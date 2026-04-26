//! Field lines renderer (§11.8) — emits cetz markup with numerically-traced
//! streamlines.

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::eval_num;
use super::spec::{Field, FieldSource};
use super::style::{Color, LineStyle};

const STEP: f64 = 0.05;
const MAX_STEPS: usize = 600;
const STREAMLINES_PER_CHARGE: usize = 16;

pub fn render(spec: &Field, vars: &VarMap) -> Result<String, DslError> {
    let xlo = eval_num(&spec.region[0], vars)?;
    let xhi = eval_num(&spec.region[1], vars)?;
    let ylo = eval_num(&spec.region[2], vars)?;
    let yhi = eval_num(&spec.region[3], vars)?;
    if xhi <= xlo || yhi <= ylo {
        return Err(DslError::Evaluation("field region must be ascending".into()));
    }

    struct Q { x: f64, y: f64, q: f64, label: Option<String> }
    let mut charges: Vec<Q> = Vec::new();
    for src in &spec.sources {
        let FieldSource::Charge(c) = src;
        let qx = eval_num(&c.position[0], vars)?;
        let qy = eval_num(&c.position[1], vars)?;
        let qv = eval_num(&c.value, vars).unwrap_or(1.0);
        charges.push(Q { x: qx, y: qy, q: qv, label: c.label.clone() });
    }

    let field = |x: f64, y: f64| {
        let mut ex = 0.0; let mut ey = 0.0;
        for c in &charges {
            let dx = x - c.x; let dy = y - c.y;
            let r2 = dx * dx + dy * dy;
            if r2 < 1e-4 { continue; }
            let r = r2.sqrt();
            let f = c.q / (r2 * r);
            ex += f * dx;
            ey += f * dy;
        }
        (ex, ey)
    };

    let mut s = String::new();
    let charge_pts: Vec<(f64, f64)> = charges.iter().map(|c| (c.x, c.y)).collect();

    if spec.show_lines {
        for c in &charges {
            for k in 0..STREAMLINES_PER_CHARGE {
                let theta = std::f64::consts::TAU * (k as f64) / (STREAMLINES_PER_CHARGE as f64);
                let x0 = c.x + 0.15 * theta.cos();
                let y0 = c.y + 0.15 * theta.sin();
                let dir = if c.q >= 0.0 { 1.0 } else { -1.0 };
                let line = trace(&field, x0, y0, dir, xlo, xhi, ylo, yhi, &charge_pts);
                if line.len() < 2 { continue; }
                cetz::polyline(&mut s, &line, Color::Gray, LineStyle::Solid);
            }
        }
    }

    for c in &charges {
        let color = if c.q >= 0.0 { Color::Red } else { Color::Blue };
        cetz::circle(&mut s, (c.x, c.y), 0.18, color, Some(color));
        let glyph = if c.q >= 0.0 { "+" } else { "-" };
        cetz::content(&mut s, (c.x, c.y), glyph);
        if let Some(label) = &c.label {
            cetz::content_anchor(&mut s, (c.x + 0.3, c.y - 0.3), label, "north-west");
        }
    }

    Ok(s)
}

fn trace<F: Fn(f64, f64) -> (f64, f64)>(
    field: F,
    mut x: f64, mut y: f64, sign: f64,
    xlo: f64, xhi: f64, ylo: f64, yhi: f64,
    charge_pts: &[(f64, f64)],
) -> Vec<(f64, f64)> {
    let mut pts = vec![(x, y)];
    for _ in 0..MAX_STEPS {
        let (ex, ey) = field(x, y);
        let mag = (ex * ex + ey * ey).sqrt();
        if mag < 1e-9 { break; }
        x += sign * STEP * ex / mag;
        y += sign * STEP * ey / mag;
        if x < xlo || x > xhi || y < ylo || y > yhi { break; }
        if charge_pts.iter().any(|(cx, cy)| {
            let dx = x - cx; let dy = y - cy;
            (dx * dx + dy * dy).sqrt() < 0.18
        }) { pts.push((x, y)); break; }
        pts.push((x, y));
    }
    pts
}
