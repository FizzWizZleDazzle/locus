//! Function graph renderer (§11.6) — emits cetz markup.

use locus_common::symengine::Expr;

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::eval_num;
use super::spec::{Feature, FunctionGraph};
use super::style::{Color, LineStyle};

const SAMPLES: usize = 200;

pub fn render(spec: &FunctionGraph, vars: &VarMap) -> Result<String, DslError> {
    let xlo = eval_num(&spec.x_range[0], vars)?;
    let xhi = eval_num(&spec.x_range[1], vars)?;
    let ylo = eval_num(&spec.y_range[0], vars)?;
    let yhi = eval_num(&spec.y_range[1], vars)?;
    if xhi <= xlo || yhi <= ylo {
        return Err(DslError::Evaluation("function_graph ranges must be ascending".into()));
    }

    let mut s = String::new();
    cetz::line(&mut s, (xlo, 0.0), (xhi, 0.0), Color::Black, LineStyle::Solid);
    cetz::line(&mut s, (0.0, ylo), (0.0, yhi), Color::Black, LineStyle::Solid);

    let mut samples_by_label: std::collections::HashMap<String, Vec<(f64, f64)>> = Default::default();
    for fe in &spec.functions {
        let expr_str = vars.get(&fe.expr).cloned().unwrap_or_else(|| fe.expr.clone());
        let parsed = Expr::parse(&expr_str)
            .map_err(|e| DslError::ExpressionParse(format!("{e}: '{expr_str}'")))?;

        let mut all_pts: Vec<(f64, f64)> = Vec::new();
        let mut clipped: Vec<(f64, f64)> = Vec::new();
        for i in 0..=SAMPLES {
            let x = xlo + (xhi - xlo) * (i as f64) / (SAMPLES as f64);
            if let Some(y) = parsed.subs_float("x", x).to_float() {
                if y.is_finite() {
                    all_pts.push((x, y));
                    if (ylo..=yhi).contains(&y) {
                        clipped.push((x, y));
                    } else if !clipped.is_empty() {
                        cetz::polyline(&mut s, &clipped, fe.color, LineStyle::Solid);
                        clipped.clear();
                    }
                }
            }
        }
        if !clipped.is_empty() {
            cetz::polyline(&mut s, &clipped, fe.color, LineStyle::Solid);
        }

        if let Some(label) = &fe.label {
            if let Some(&(x, y)) = all_pts.iter().rev().find(|p| (ylo..=yhi).contains(&p.1)) {
                cetz::content_anchor(&mut s, (x, y), label, "north-east");
            }
            samples_by_label.insert(label.clone(), all_pts);
        } else {
            samples_by_label.insert(fe.expr.clone(), all_pts);
        }
    }

    for feat in &spec.features {
        let (kind, fb) = match feat {
            Feature::Zero(b) => ("zero", b),
            Feature::Maximum(b) => ("max", b),
            Feature::Minimum(b) => ("min", b),
            Feature::Inflection(b) => ("infl", b),
        };
        let pts = match samples_by_label.get(&fb.of) { Some(p) => p, None => continue };
        for (x, y) in find_features(pts, kind) {
            if !(xlo..=xhi).contains(&x) || !(ylo..=yhi).contains(&y) { continue; }
            cetz::point(&mut s, (x, y), Color::Red);
            if fb.label {
                let lab = format!("({:.0}, {:.0})", x, y);
                // Place label diagonally to the upper-right, away from the
                // curve direction, with bigger clearance.
                let dx_off = (xhi - xlo) * 0.05;
                let dy_off = (yhi - ylo) * 0.07;
                let nudge = if matches!(kind, "min") { -dy_off } else { dy_off };
                let label_at = (x + dx_off, (y + nudge).clamp(ylo, yhi));
                let anchor = if nudge >= 0.0 { "south-west" } else { "north-west" };
                cetz::content_anchor(&mut s, label_at, &lab, anchor);
            }
        }
    }

    Ok(s)
}

fn find_features(pts: &[(f64, f64)], kind: &str) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    if pts.len() < 3 { return out; }
    match kind {
        "zero" => {
            for w in pts.windows(2) {
                if w[0].1 * w[1].1 < 0.0 {
                    let (x0, y0) = w[0]; let (x1, y1) = w[1];
                    let frac = -y0 / (y1 - y0);
                    out.push((x0 + (x1 - x0) * frac, 0.0));
                }
            }
        }
        "max" => for w in pts.windows(3) { if w[1].1 > w[0].1 && w[1].1 > w[2].1 { out.push(w[1]); } },
        "min" => for w in pts.windows(3) { if w[1].1 < w[0].1 && w[1].1 < w[2].1 { out.push(w[1]); } },
        "infl" => {
            let n = pts.len();
            for i in 1..n - 1 {
                let s1 = pts[i].1 - pts[i - 1].1;
                let s2 = pts[i + 1].1 - pts[i].1;
                let next = if i + 2 < n { pts[i + 2].1 - pts[i + 1].1 } else { s2 };
                if (s2 - s1) * (next - s2) < 0.0 { out.push(pts[i]); }
            }
        }
        _ => {}
    }
    out
}
