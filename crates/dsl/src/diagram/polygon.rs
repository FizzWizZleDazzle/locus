//! Polygon renderer (§11.4) — emits cetz markup.

use std::collections::BTreeMap;

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::eval_num;
use super::spec::{AngleValue, Polygon, SideValue, Vertex};
use super::style::{Color, LineStyle};

pub fn render(spec: &Polygon, vars: &VarMap) -> Result<String, DslError> {
    let n = spec.vertices.len();
    if n < 3 {
        return Err(DslError::Evaluation("polygon requires at least 3 vertices".into()));
    }
    let names: Vec<String> = spec.vertices.iter().enumerate().map(|(i, v)| v.name(i)).collect();

    let explicit_coords: Option<Vec<(f64, f64)>> = spec.vertices.iter().map(|v| match v {
        Vertex::Coord { x, y, .. } => {
            let xv = eval_num(x.as_str(), vars).ok()?;
            let yv = eval_num(y.as_str(), vars).ok()?;
            Some((xv, yv))
        }
        Vertex::Named(_) => None,
    }).collect();

    let pts: Vec<(f64, f64)> = if let Some(p) = explicit_coords { p } else {
        compute_layout(spec, &names, vars)?
    };

    let mut s = String::new();
    cetz::polygon(&mut s, &pts, Color::Black, None);

    // Side labels and style overrides.
    for i in 0..n {
        let a = &names[i];
        let b = &names[(i + 1) % n];
        if let Some(side) = lookup_side(spec, a, b) {
            let p1 = pts[i]; let p2 = pts[(i + 1) % n];
            if !matches!(side.style(), LineStyle::Solid) {
                cetz::line(&mut s, p1, p2, Color::Black, side.style());
            }
            if let Some(label) = side.label() {
                let mid = ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0);
                let centroid = centroid(&pts);
                let dx = centroid.0 - mid.0; let dy = centroid.1 - mid.1;
                let len = (dx * dx + dy * dy).sqrt().max(1e-6);
                let off = 0.25;
                let lp = (mid.0 - dx / len * off, mid.1 - dy / len * off);
                cetz::content(&mut s, lp, label);
            }
        }
    }

    // Vertex labels.
    let cen = centroid(&pts);
    for (name, p) in names.iter().zip(pts.iter()) {
        let dx = p.0 - cen.0; let dy = p.1 - cen.1;
        let len = (dx * dx + dy * dy).sqrt().max(1e-6);
        let off = 0.3;
        let lp = (p.0 + dx / len * off, p.1 + dy / len * off);
        cetz::content(&mut s, lp, name);
    }

    // Right-angle marks.
    for name in &names {
        if let Some(AngleValue::Marker(m)) = spec.angles.get(name) {
            if m == "right_angle" {
                if let Some(idx) = names.iter().position(|x| x == name) {
                    draw_right_angle(&mut s, &pts, idx);
                }
            }
        }
    }

    // Center label.
    if let Some(text) = spec.labels.get("center") {
        cetz::content(&mut s, cen, text);
    }

    Ok(s)
}

fn compute_layout(spec: &Polygon, names: &[String], vars: &VarMap) -> Result<Vec<(f64, f64)>, DslError> {
    let n = names.len();
    let mut lengths: Vec<f64> = Vec::with_capacity(n);
    let mut total = 0.0; let mut counted = 0usize;
    for i in 0..n {
        let len = lookup_side(spec, &names[i], &names[(i + 1) % n])
            .and_then(|s| s.length())
            .and_then(|e| eval_num(e, vars).ok());
        if let Some(v) = len { total += v; counted += 1; }
        lengths.push(len.unwrap_or(0.0));
    }
    let default_len = if counted > 0 { total / counted as f64 } else { 1.0 };
    for l in lengths.iter_mut() { if *l <= 0.0 { *l = default_len; } }

    let regular = ((n as f64 - 2.0) * 180.0) / n as f64;
    let mut interior: Vec<f64> = Vec::with_capacity(n);
    for v in names {
        let deg = match spec.angles.get(v) {
            Some(AngleValue::Numeric(num)) => eval_num(num, vars).unwrap_or(regular),
            Some(AngleValue::Marker(m)) if m == "right_angle" => 90.0,
            _ => regular,
        };
        interior.push(deg);
    }
    let target = (n as f64 - 2.0) * 180.0;
    let actual: f64 = interior.iter().sum();
    if actual > 0.0 {
        let scale = target / actual;
        for d in interior.iter_mut() { *d *= scale; }
    }

    let mut pts: Vec<(f64, f64)> = vec![(0.0, 0.0)];
    let mut heading = 0.0_f64;
    for i in 0..(n - 1) {
        let (cx, cy) = pts[i];
        pts.push((cx + lengths[i] * heading.cos(), cy + lengths[i] * heading.sin()));
        let turn = (180.0 - interior[(i + 1) % n]).to_radians();
        heading += turn;
    }
    Ok(pts)
}

fn lookup_side<'a>(spec: &'a Polygon, a: &str, b: &str) -> Option<&'a SideValue> {
    spec.sides.get(&format!("{a}{b}")).or_else(|| spec.sides.get(&format!("{b}{a}")))
}

fn centroid(pts: &[(f64, f64)]) -> (f64, f64) {
    let n = pts.len() as f64;
    let sx: f64 = pts.iter().map(|p| p.0).sum();
    let sy: f64 = pts.iter().map(|p| p.1).sum();
    (sx / n, sy / n)
}

fn draw_right_angle(buf: &mut String, pts: &[(f64, f64)], idx: usize) {
    let n = pts.len();
    let p = pts[idx];
    let prev = pts[(idx + n - 1) % n];
    let next = pts[(idx + 1) % n];
    let v1 = unit(sub(prev, p));
    let v2 = unit(sub(next, p));
    let s = 0.2;
    let a = (p.0 + v1.0 * s, p.1 + v1.1 * s);
    let b = (p.0 + v2.0 * s, p.1 + v2.1 * s);
    let c = (a.0 + v2.0 * s, a.1 + v2.1 * s);
    cetz::line(buf, a, c, Color::Black, LineStyle::Solid);
    cetz::line(buf, c, b, Color::Black, LineStyle::Solid);
}

fn sub(a: (f64, f64), b: (f64, f64)) -> (f64, f64) { (a.0 - b.0, a.1 - b.1) }
fn unit(v: (f64, f64)) -> (f64, f64) {
    let len = (v.0 * v.0 + v.1 * v.1).sqrt().max(1e-6);
    (v.0 / len, v.1 / len)
}

#[allow(dead_code)]
fn _suppress() { let _: BTreeMap<String, String> = BTreeMap::new(); }
