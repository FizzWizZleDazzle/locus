//! Circle renderer (§11.3) — emits cetz markup. Points are auto-distributed
//! evenly around a unit circle (math units); explicit `point: {name, angle}`
//! overrides position.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::{eval_num, format_label};
use super::spec::{Circle, CircleElement};
use super::style::{Color, LineStyle};

pub fn render(spec: &Circle, _vars: &VarMap) -> Result<String, DslError> {
    let r = 1.0_f64; // math-unit radius; cetz autoscales

    let mut named_pts: Vec<String> = Vec::new();
    let mut explicit: BTreeMap<String, f64> = BTreeMap::new();
    let push = |v: &mut Vec<String>, n: &str| {
        if !v.iter().any(|x| x == n) { v.push(n.to_string()); }
    };
    for el in &spec.elements {
        match el {
            CircleElement::Chord(c) => { push(&mut named_pts, &c.from); push(&mut named_pts, &c.to); }
            CircleElement::Arc(a) => { push(&mut named_pts, &a.from); push(&mut named_pts, &a.to); }
            CircleElement::Radius(rd) => { push(&mut named_pts, &rd.to); }
            CircleElement::Tangent(t) => { push(&mut named_pts, &t.at); }
            CircleElement::CentralAngle(c) => {
                push(&mut named_pts, &c.vertex);
                push(&mut named_pts, &c.sides[0]);
                push(&mut named_pts, &c.sides[1]);
            }
            CircleElement::InscribedAngle(i) => {
                push(&mut named_pts, &i.vertex);
                push(&mut named_pts, &i.sides[0]);
                push(&mut named_pts, &i.sides[1]);
            }
            CircleElement::Point(p) => {
                push(&mut named_pts, &p.name);
                explicit.insert(p.name.clone(), eval_num(&p.angle, _vars).unwrap_or(0.0).to_radians());
            }
        }
    }

    let center = &spec.center;
    let on_circle: Vec<&String> = named_pts.iter().filter(|n| **n != *center).collect();
    let n_auto: usize = on_circle.iter().filter(|n| !explicit.contains_key(n.as_str())).count();
    let auto_step = if n_auto > 0 { TAU / (n_auto as f64) } else { 0.0 };
    let mut auto_idx = 0usize;
    let mut positions: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    positions.insert(center.clone(), (0.0, 0.0));
    for n in &on_circle {
        let theta = match explicit.get(n.as_str()) {
            Some(&t) => t,
            None => {
                let t = PI / 2.0 - (auto_idx as f64) * auto_step;
                auto_idx += 1;
                t
            }
        };
        positions.insert((*n).clone(), (r * theta.cos(), r * theta.sin()));
    }

    let mut s = String::new();
    cetz::circle(&mut s, (0.0, 0.0), r, Color::Black, None);
    cetz::point(&mut s, (0.0, 0.0), Color::Black);
    cetz::content_anchor(&mut s, (0.0, 0.0), center, "south-east");

    let put_pt = |s: &mut String, name: &str, positions: &BTreeMap<String, (f64, f64)>| {
        let p = positions[name];
        cetz::point(s, p, Color::Black);
        let dx = p.0; let dy = p.1;
        let len = (dx * dx + dy * dy).sqrt().max(1e-6);
        let off = 0.18;
        let lp = (p.0 + dx / len * off, p.1 + dy / len * off);
        cetz::content(s, lp, name);
    };

    for el in &spec.elements {
        match el {
            CircleElement::Chord(c) => {
                let p = positions[&c.from]; let q = positions[&c.to];
                cetz::line(&mut s, p, q, c.color, LineStyle::Solid);
                if let Some(label) = &c.label {
                    let mid = ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
                    cetz::content(&mut s, mid, &format_label(label, _vars, ""));
                }
                put_pt(&mut s, &c.from, &positions);
                put_pt(&mut s, &c.to, &positions);
            }
            CircleElement::Arc(a) => {
                let p1 = positions[&a.from]; let p2 = positions[&a.to];
                let theta1 = p1.1.atan2(p1.0);
                let theta2 = p2.1.atan2(p2.0);
                let r_arc = r * 0.85;
                cetz_arc(&mut s, (0.0, 0.0), r_arc, theta1.to_degrees(), theta2.to_degrees());
                if let Some(label) = &a.label {
                    let mid = (theta1 + theta2) / 2.0;
                    cetz::content(&mut s, (r * 0.7 * mid.cos(), r * 0.7 * mid.sin()), &format_label(label, _vars, ""));
                }
            }
            CircleElement::Radius(rd) => {
                let p = positions[&rd.to];
                cetz::line(&mut s, (0.0, 0.0), p, Color::Black, LineStyle::Solid);
                if let Some(label) = &rd.label {
                    let mid = (p.0 / 2.0, p.1 / 2.0);
                    cetz::content(&mut s, mid, &format_label(label, _vars, ""));
                }
                put_pt(&mut s, &rd.to, &positions);
            }
            CircleElement::Tangent(t) => {
                let p = positions[&t.at];
                let theta = p.1.atan2(p.0);
                let tx = -theta.sin();
                let ty = theta.cos();
                let len = r * 0.6;
                cetz::line(
                    &mut s,
                    (p.0 - tx * len, p.1 - ty * len),
                    (p.0 + tx * len, p.1 + ty * len),
                    Color::Gray, LineStyle::Solid,
                );
                if let Some(label) = &t.label {
                    cetz::content(&mut s, (p.0 + tx * len, p.1 + ty * len), &format_label(label, _vars, ""));
                }
                put_pt(&mut s, &t.at, &positions);
            }
            CircleElement::CentralAngle(c) => {
                let v = positions[&c.vertex];
                let a = positions[&c.sides[0]];
                let b = positions[&c.sides[1]];
                cetz::line(&mut s, v, a, Color::Black, LineStyle::Solid);
                cetz::line(&mut s, v, b, Color::Black, LineStyle::Solid);
                let lbl = c.label.as_deref().map(|l| format_label(l, _vars, "°"))
                    .unwrap_or_default();
                cetz::angle_arc(&mut s, v, a, b, &lbl, 0.2);
                put_pt(&mut s, &c.sides[0], &positions);
                put_pt(&mut s, &c.sides[1], &positions);
            }
            CircleElement::InscribedAngle(i) => {
                let v = positions[&i.vertex];
                let a = positions[&i.sides[0]];
                let b = positions[&i.sides[1]];
                cetz::line(&mut s, v, a, Color::Blue, LineStyle::Solid);
                cetz::line(&mut s, v, b, Color::Blue, LineStyle::Solid);
                let lbl = i.label.as_deref().map(|l| format_label(l, _vars, "°"))
                    .unwrap_or_default();
                cetz::angle_arc(&mut s, v, a, b, &lbl, 0.18);
                put_pt(&mut s, &i.vertex, &positions);
                put_pt(&mut s, &i.sides[0], &positions);
                put_pt(&mut s, &i.sides[1], &positions);
            }
            CircleElement::Point(p) => {
                put_pt(&mut s, &p.name, &positions);
                if let Some(lab) = &p.label {
                    cetz::content(&mut s, positions[&p.name], lab);
                }
            }
        }
    }

    Ok(s)
}

fn cetz_arc(buf: &mut String, center: (f64, f64), r: f64, start_deg: f64, end_deg: f64) {
    use std::fmt::Write;
    let _ = write!(
        buf,
        "arc({}, radius: {}, start: {}deg, stop: {}deg, anchor: \"origin\")\n",
        cetz::pt(center.0, center.1),
        cetz::n(r),
        cetz::n(start_deg),
        cetz::n(end_deg),
    );
}

fn mid_angle(a: f64, b: f64) -> f64 {
    let mut diff = b - a;
    while diff > PI { diff -= TAU; }
    while diff < -PI { diff += TAU; }
    a + diff / 2.0
}
