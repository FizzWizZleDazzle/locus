//! Coordinate plane renderer (§11.1) — emits cetz markup using the
//! `cetz.draw` axes + `cetz.plot` style primitives.

use std::fmt::Write;

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::{eval_num, eval_num_opt};
use super::spec::{CoordinatePlane, CpElement};
use super::style::{Color, LineStyle};

pub fn render(spec: &CoordinatePlane, vars: &VarMap) -> Result<String, DslError> {
    let xlo = eval_num(&spec.x_range[0], vars)?;
    let xhi = eval_num(&spec.x_range[1], vars)?;
    let ylo = eval_num(&spec.y_range[0], vars)?;
    let yhi = eval_num(&spec.y_range[1], vars)?;
    if xhi <= xlo || yhi <= ylo {
        return Err(DslError::Evaluation(
            "coordinate_plane ranges must be ascending".into(),
        ));
    }

    let mut s = String::new();

    // Grid (light gray).
    if spec.grid {
        for i in (xlo.ceil() as i64)..=(xhi.floor() as i64) {
            if i == 0 { continue; }
            let x = i as f64;
            let _ = write!(
                s,
                "line({}, {}, stroke: (paint: gray, thickness: 0.3pt))\n",
                cetz::pt(x, ylo), cetz::pt(x, yhi),
            );
        }
        for i in (ylo.ceil() as i64)..=(yhi.floor() as i64) {
            if i == 0 { continue; }
            let y = i as f64;
            let _ = write!(
                s,
                "line({}, {}, stroke: (paint: gray, thickness: 0.3pt))\n",
                cetz::pt(xlo, y), cetz::pt(xhi, y),
            );
        }
    }

    // Axes.
    cetz::line(&mut s, (xlo, 0.0), (xhi, 0.0), Color::Black, LineStyle::Solid);
    cetz::line(&mut s, (0.0, ylo), (0.0, yhi), Color::Black, LineStyle::Solid);

    // Tick labels.
    for i in (xlo.ceil() as i64)..=(xhi.floor() as i64) {
        if i == 0 { continue; }
        cetz::content_anchor(&mut s, (i as f64, -0.3), &i.to_string(), "north");
    }
    for i in (ylo.ceil() as i64)..=(yhi.floor() as i64) {
        if i == 0 { continue; }
        cetz::content_anchor(&mut s, (-0.3, i as f64), &i.to_string(), "east");
    }

    let mut named_lines: std::collections::HashMap<String, (f64, f64)> = Default::default();

    for el in &spec.elements {
        match el {
            CpElement::Line(l) => {
                let m = eval_num(&l.slope, vars)?;
                let b = eval_num(&l.intercept, vars)?;
                if let Some(((x1, y1), (x2, y2))) = clip_line(m, b, xlo, xhi, ylo, yhi) {
                    cetz::line(&mut s, (x1, y1), (x2, y2), l.color, l.style);
                    if let Some(label) = &l.label {
                        cetz::content_anchor(&mut s, (x2, y2), label, "south-east");
                        named_lines.insert(label.clone(), (m, b));
                    }
                }
            }
            CpElement::Point(p) => {
                let x = eval_num(&p.x, vars)?;
                let y = eval_num(&p.y, vars)?;
                cetz::point(&mut s, (x, y), p.color);
                if let Some(label) = &p.label {
                    cetz::content_anchor(&mut s, (x, y), label, "north-west");
                }
            }
            CpElement::Shade(sh) => {
                let f = eval_num(&sh.from, vars)?;
                let t = eval_num(&sh.to, vars)?;
                if let (Some(&(m1, b1)), Some(&(m2, b2))) = (
                    named_lines.get(&sh.between[0]),
                    named_lines.get(&sh.between[1]),
                ) {
                    let mut pts: Vec<(f64, f64)> = Vec::new();
                    for i in 0..=24 {
                        let x = f + (t - f) * (i as f64) / 24.0;
                        pts.push((x, m1 * x + b1));
                    }
                    for i in (0..=24).rev() {
                        let x = f + (t - f) * (i as f64) / 24.0;
                        pts.push((x, m2 * x + b2));
                    }
                    cetz::polygon(&mut s, &pts, sh.color, Some(sh.color));
                }
            }
            CpElement::Asymptote(a) => {
                let x_opt = eval_num_opt(a.x.as_deref(), vars)?;
                let y_opt = eval_num_opt(a.y.as_deref(), vars)?;
                if let Some(x) = x_opt {
                    cetz::line(&mut s, (x, ylo), (x, yhi), Color::Gray, LineStyle::Dashed);
                }
                if let Some(y) = y_opt {
                    cetz::line(&mut s, (xlo, y), (xhi, y), Color::Gray, LineStyle::Dashed);
                }
            }
            CpElement::Arrow(a) => {
                let x1 = eval_num(&a.from[0], vars)?;
                let y1 = eval_num(&a.from[1], vars)?;
                let x2 = eval_num(&a.to[0], vars)?;
                let y2 = eval_num(&a.to[1], vars)?;
                cetz::arrow(&mut s, (x1, y1), (x2, y2), a.color);
                if let Some(label) = &a.label {
                    cetz::content_anchor(&mut s, (x2, y2), label, "north-west");
                }
            }
        }
    }

    Ok(s)
}

fn clip_line(m: f64, b: f64, xlo: f64, xhi: f64, ylo: f64, yhi: f64) -> Option<((f64, f64), (f64, f64))> {
    let mut pts: Vec<(f64, f64)> = Vec::new();
    let push = |pts: &mut Vec<(f64, f64)>, p: (f64, f64)| {
        if !pts.iter().any(|q| (q.0 - p.0).abs() < 1e-9 && (q.1 - p.1).abs() < 1e-9) {
            pts.push(p);
        }
    };
    let y_at = |x: f64| m * x + b;
    if (ylo..=yhi).contains(&y_at(xlo)) { push(&mut pts, (xlo, y_at(xlo))); }
    if (ylo..=yhi).contains(&y_at(xhi)) { push(&mut pts, (xhi, y_at(xhi))); }
    if m.abs() > 1e-9 {
        let x_at = |y: f64| (y - b) / m;
        if (xlo..=xhi).contains(&x_at(ylo)) { push(&mut pts, (x_at(ylo), ylo)); }
        if (xlo..=xhi).contains(&x_at(yhi)) { push(&mut pts, (x_at(yhi), yhi)); }
    }
    if pts.len() >= 2 { Some((pts[0], pts[1])) } else { None }
}
