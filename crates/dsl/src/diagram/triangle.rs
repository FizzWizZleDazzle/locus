//! Triangle renderer (§11.2) — emits cetz markup.

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::{eval_num, format_label};
use super::spec::{AngleValue, SideValue, Triangle};
use super::style::{Color, LineStyle};

pub fn render(spec: &Triangle, vars: &VarMap) -> Result<String, DslError> {
    let v = &spec.vertices;
    let a = &v[0]; let b = &v[1]; let c = &v[2];

    // Right-angle marker (either `right_angle: <name>` or `angles: {<name>: right_angle}`)
    // forces a Pythagorean layout with the two legs given as the sides
    // adjacent to the right-angle vertex.
    let right_at = right_angle_vertex(spec);

    let (pa0, pb0, pc0) = if let Some(right_v) = right_at.as_deref() {
        layout_right_triangle(spec, [a.as_str(), b.as_str(), c.as_str()], right_v, vars)
    } else {
        layout_general(spec, a, b, c, vars)
    };
    // Normalize per-axis: each dimension scaled so the longer axis fills
    // TARGET_LONG and the shorter at least MIN_SHORT. Keeps tall-thin or
    // wide-flat triangles from collapsing to a sliver in a square panel
    // while still preserving overall shape proportions.
    const TARGET_LONG: f64 = 6.0;
    const MIN_SHORT: f64 = 4.0;
    let (pa, pb, pc) = normalize_proportional([pa0, pb0, pc0], TARGET_LONG, MIN_SHORT);

    let mut s = String::new();
    let pts = [pa, pb, pc];
    let centroid = ((pa.0 + pb.0 + pc.0) / 3.0, (pa.1 + pb.1 + pc.1) / 3.0);

    // Three sides as named cetz lines so labels can ride along them
    // perpendicular to the edge using path-interpolation anchors.
    let edges = [
        ("ab", 0usize, 1usize, a.as_str(), b.as_str()),
        ("bc", 1, 2, b.as_str(), c.as_str()),
        ("ca", 2, 0, c.as_str(), a.as_str()),
    ];
    for (name, i, j, n1, n2) in edges {
        let side = lookup_side(spec, n1, n2);
        let style = side.map(|s| s.style()).unwrap_or(LineStyle::Solid);
        cetz::line_named(&mut s, name, pts[i], pts[j], Color::Black, style);
        if let Some(side) = side {
            let outward_anchor = perpendicular_anchor(pts[i], pts[j], centroid);
            if let Some(label) = side.label() {
                cetz::label_on_line(&mut s, name, &format_label(label, vars, ""), outward_anchor);
            } else if let Some(len_expr) = side.length() {
                cetz::label_on_line(&mut s, name, &format_label(len_expr, vars, ""), outward_anchor);
            }
        }
    }

    // Vertex labels — push outward from centroid.
    for (vname, p) in v.iter().zip(pts.iter()) {
        let dx = p.0 - centroid.0; let dy = p.1 - centroid.1;
        let len = (dx * dx + dy * dy).sqrt().max(1e-6);
        let off = 0.4;
        let lp = (p.0 + dx / len * off, p.1 + dy / len * off);
        cetz::content_plain(&mut s, lp, vname);
    }

    // Angle labels via cetz angle.angle helper (auto arc + bisector label).
    for (i, vname) in v.iter().enumerate() {
        let prev = pts[(i + 2) % 3];
        let next = pts[(i + 1) % 3];
        if let Some(angle) = spec.angles.get(vname) {
            match angle {
                AngleValue::Numeric(num) => {
                    let lbl = format_label(num.as_str(), vars, "°");
                    cetz::angle_arc(&mut s, pts[i], prev, next, &lbl, 0.5);
                }
                AngleValue::Marker(m) if m == "right_angle" => {
                    cetz::right_angle_mark(&mut s, pts[i], prev, next, 0.35);
                }
                AngleValue::Marker(m) => {
                    cetz::angle_arc(&mut s, pts[i], prev, next, m, 0.5);
                }
            }
        }
    }

    // Top-level right_angle: marker too.
    if let Some(ra) = &spec.right_angle {
        if let Some(idx) = v.iter().position(|n| n == ra) {
            let prev = pts[(idx + 2) % 3];
            let next = pts[(idx + 1) % 3];
            cetz::right_angle_mark(&mut s, pts[idx], prev, next, 0.35);
        }
    }

    Ok(s)
}

/// Pick `"south"` or `"north"` for a side label depending on which side of
/// the line the centroid lies. Labels go AWAY from centroid (outside polygon).
fn perpendicular_anchor(p1: (f64, f64), p2: (f64, f64), centroid: (f64, f64)) -> &'static str {
    // Cross product (line direction) × (mid -> centroid) determines which
    // side. Positive = centroid above the directed line, so label goes south.
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let mx = (p1.0 + p2.0) / 2.0;
    let my = (p1.1 + p2.1) / 2.0;
    let cross = dx * (centroid.1 - my) - dy * (centroid.0 - mx);
    if cross > 0.0 { "south" } else { "north" }
}

/// Normalize so that the longer bbox axis spans `long` units and the shorter
/// is at least `min_short` units (mild aspect-ratio compression to avoid
/// thin slivers in a square panel). Preserves shape direction; this is purely
/// a visual fit, not a metric scaling.
fn normalize_proportional(
    pts: [(f64, f64); 3],
    long: f64,
    min_short: f64,
) -> ((f64, f64), (f64, f64), (f64, f64)) {
    let xs = [pts[0].0, pts[1].0, pts[2].0];
    let ys = [pts[0].1, pts[1].1, pts[2].1];
    let xmin = xs.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let ymin = ys.iter().cloned().fold(f64::INFINITY, f64::min);
    let ymax = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let dx = (xmax - xmin).max(1e-6);
    let dy = (ymax - ymin).max(1e-6);
    let scale = long / dx.max(dy);
    let new_dx = dx * scale;
    let new_dy = dy * scale;
    let sx_extra = if new_dx < min_short { min_short / new_dx } else { 1.0 };
    let sy_extra = if new_dy < min_short { min_short / new_dy } else { 1.0 };
    let cx = (xmin + xmax) / 2.0;
    let cy = (ymin + ymax) / 2.0;
    let map = |p: (f64, f64)| ((p.0 - cx) * scale * sx_extra, (p.1 - cy) * scale * sy_extra);
    (map(pts[0]), map(pts[1]), map(pts[2]))
}

fn right_angle_vertex(spec: &Triangle) -> Option<String> {
    if let Some(v) = &spec.right_angle { return Some(v.clone()); }
    for (k, av) in &spec.angles {
        if let AngleValue::Marker(m) = av {
            if m == "right_angle" { return Some(k.clone()); }
        }
    }
    None
}

/// Layout a right triangle with the right angle at `right_v`. The two legs
/// are the sides adjacent to that vertex; if their lengths are missing, fall
/// back to a 1:1 ratio so the picture is at least visibly a right triangle.
/// Returns positions for vertices in the original (a, b, c) order.
fn layout_right_triangle(
    spec: &Triangle,
    names: [&str; 3],
    right_v: &str,
    vars: &VarMap,
) -> ((f64, f64), (f64, f64), (f64, f64)) {
    // Identify right vertex + its two neighbors.
    let r_idx = names.iter().position(|n| *n == right_v).unwrap_or(1);
    let n1 = names[(r_idx + 1) % 3];
    let n2 = names[(r_idx + 2) % 3];

    // Hypotenuse is the side opposite the right angle (between n1 and n2).
    let leg1 = side_len(spec, right_v, n1, vars);
    let leg2 = side_len(spec, right_v, n2, vars);
    let hyp = side_len(spec, n1, n2, vars);

    let alpha = angle_deg(spec, n1, vars)
        .map(|d| d.to_radians())
        .unwrap_or(std::f64::consts::FRAC_PI_4);
    // Choose leg lengths from whatever's available.
    let (l1, l2) = if let (Some(a), Some(b)) = (leg1, leg2) {
        (a, b)
    } else if let (Some(a), Some(h)) = (leg1, hyp) {
        let other = (h * h - a * a).max(0.0).sqrt();
        (a, if other > 0.0 { other } else { a })
    } else if let (Some(b), Some(h)) = (leg2, hyp) {
        let other = (h * h - b * b).max(0.0).sqrt();
        (if other > 0.0 { other } else { b }, b)
    } else if let Some(h) = hyp {
        (h * alpha.cos(), h * alpha.sin())
    } else if let Some(a) = leg1 {
        (a, a * alpha.tan().max(0.5))
    } else if let Some(b) = leg2 {
        (b / alpha.tan().max(0.5), b)
    } else {
        (alpha.cos(), alpha.sin())
    };

    // Place right vertex at origin, leg1 along +x, leg2 along +y.
    let p_right = (0.0, 0.0);
    let p_n1 = (l1, 0.0);
    let p_n2 = (0.0, l2);

    // Map back to (a, b, c) original order.
    let mut out = [(0.0, 0.0); 3];
    out[r_idx] = p_right;
    out[(r_idx + 1) % 3] = p_n1;
    out[(r_idx + 2) % 3] = p_n2;
    (out[0], out[1], out[2])
}

fn layout_general(
    spec: &Triangle, a: &str, b: &str, c: &str, vars: &VarMap,
) -> ((f64, f64), (f64, f64), (f64, f64)) {
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
    ((0.0, 0.0), (ab, 0.0), (ac * theta.cos(), ac * theta.sin()))
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
