//! Triangle renderer (§11.2) — emits cetz markup.

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::eval_num;
use super::spec::{AngleValue, SideValue, Triangle};
use super::style::{Color, LineStyle};

pub fn render(spec: &Triangle, vars: &VarMap) -> Result<String, DslError> {
    let v = &spec.vertices;
    let a = &v[0]; let b = &v[1]; let c = &v[2];

    let ab = side_len(spec, a, b, vars).unwrap_or(1.0);
    let ac = side_len(spec, a, c, vars).unwrap_or(1.0);
    let bc = side_len(spec, b, c, vars);

    let angle_a_deg = angle_deg(spec, a, vars).unwrap_or_else(|| {
        if let Some(bc_v) = bc {
            let cos_a = (ab * ab + ac * ac - bc_v * bc_v) / (2.0 * ab * ac);
            cos_a.clamp(-1.0, 1.0).acos().to_degrees()
        } else { 60.0 }
    });
    let theta = angle_a_deg.to_radians();

    let pa = (0.0, 0.0);
    let pb = (ab, 0.0);
    let pc = (ac * theta.cos(), ac * theta.sin());

    let mut s = String::new();
    cetz::polygon(&mut s, &[pa, pb, pc], Color::Black, None);

    // Vertex labels (offset outward from centroid).
    let centroid = ((pa.0 + pb.0 + pc.0) / 3.0, (pa.1 + pb.1 + pc.1) / 3.0);
    for (name, p) in v.iter().zip([pa, pb, pc].iter()) {
        let dx = p.0 - centroid.0;
        let dy = p.1 - centroid.1;
        let len = (dx * dx + dy * dy).sqrt().max(1e-6);
        let off = 0.2;
        let lp = (p.0 + dx / len * off, p.1 + dy / len * off);
        cetz::content(&mut s, lp, name);
    }

    // Side overrides (style/labels).
    for (i, j, n1, n2) in [(0usize, 1usize, a, b), (1, 2, b, c), (0, 2, a, c)] {
        if let Some(side) = lookup_side(spec, n1, n2) {
            let pts = [pa, pb, pc];
            if !matches!(side.style(), LineStyle::Solid) {
                cetz::line(&mut s, pts[i], pts[j], Color::Black, side.style());
            }
            let mid = ((pts[i].0 + pts[j].0) / 2.0, (pts[i].1 + pts[j].1) / 2.0);
            // Outward-perpendicular offset for label.
            let dx = centroid.0 - mid.0;
            let dy = centroid.1 - mid.1;
            let len = (dx * dx + dy * dy).sqrt().max(1e-6);
            let lp = (mid.0 - dx / len * 0.2, mid.1 - dy / len * 0.2);
            if let Some(label) = side.label() {
                cetz::content(&mut s, lp, label);
            } else if let Some(len_expr) = side.length() {
                cetz::content(&mut s, lp, len_expr);
            }
        }
    }

    // Angle labels (interior, near vertex).
    let pts = [pa, pb, pc];
    for (i, name) in v.iter().enumerate() {
        if let Some(angle) = spec.angles.get(name) {
            let label = match angle {
                AngleValue::Numeric(n) => n.as_str().to_string(),
                AngleValue::Marker(m) if m == "right_angle" => continue,
                AngleValue::Marker(m) => m.clone(),
            };
            let pos = pts[i];
            let dx = centroid.0 - pos.0;
            let dy = centroid.1 - pos.1;
            let len = (dx * dx + dy * dy).sqrt().max(1e-6);
            let off = 0.35;
            let lp = (pos.0 + dx / len * off, pos.1 + dy / len * off);
            cetz::content(&mut s, lp, &label);
        }
    }

    // Right-angle squares.
    if let Some(ra) = &spec.right_angle {
        if let Some(idx) = v.iter().position(|n| n == ra) {
            draw_right_angle(&mut s, &pts, idx);
        }
    }
    for (i, name) in v.iter().enumerate() {
        if let Some(AngleValue::Marker(m)) = spec.angles.get(name) {
            if m == "right_angle" { draw_right_angle(&mut s, &pts, i); }
        }
    }

    Ok(s)
}

fn side_len(spec: &Triangle, a: &str, b: &str, vars: &VarMap) -> Option<f64> {
    lookup_side(spec, a, b).and_then(|s| s.length()).and_then(|e| eval_num(e, vars).ok())
}

fn lookup_side<'a>(spec: &'a Triangle, a: &str, b: &str) -> Option<&'a SideValue> {
    spec.sides.get(&format!("{a}{b}")).or_else(|| spec.sides.get(&format!("{b}{a}")))
}

fn angle_deg(spec: &Triangle, vertex: &str, vars: &VarMap) -> Option<f64> {
    spec.angles.get(vertex).and_then(|a| match a {
        AngleValue::Numeric(n) => eval_num(n, vars).ok(),
        AngleValue::Marker(m) if m == "right_angle" => Some(90.0),
        _ => None,
    })
}

fn draw_right_angle(buf: &mut String, pts: &[(f64, f64)], idx: usize) {
    let n = pts.len();
    let p = pts[idx];
    let prev = pts[(idx + n - 1) % n];
    let next = pts[(idx + 1) % n];
    let v1 = unit(sub(prev, p));
    let v2 = unit(sub(next, p));
    let s = 0.15;
    let a = (p.0 + v1.0 * s, p.1 + v1.1 * s);
    let b = (p.0 + v2.0 * s, p.1 + v2.1 * s);
    let c = (a.0 + v2.0 * s, a.1 + v2.1 * s);
    use super::style::LineStyle;
    cetz::line(buf, a, c, Color::Black, LineStyle::Solid);
    cetz::line(buf, c, b, Color::Black, LineStyle::Solid);
}

fn sub(a: (f64, f64), b: (f64, f64)) -> (f64, f64) { (a.0 - b.0, a.1 - b.1) }
fn unit(v: (f64, f64)) -> (f64, f64) {
    let len = (v.0 * v.0 + v.1 * v.1).sqrt().max(1e-6);
    (v.0 / len, v.1 / len)
}
