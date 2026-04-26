//! Force diagram renderer (§11.7) — emits cetz markup.

use std::f64::consts::PI;

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::eval_num;
use super::spec::{ForceDiagram, ForceEntry, ObjectKind, SurfaceKind};
use super::style::{Color, LineStyle};

const ARROW_LEN: f64 = 1.5;

pub fn render(spec: &ForceDiagram, vars: &VarMap) -> Result<String, DslError> {
    let incline_deg = spec
        .incline_angle
        .as_ref()
        .and_then(|a| eval_num(a, vars).ok())
        .unwrap_or(30.0);
    let incline = incline_deg.to_radians();

    let mut s = String::new();

    match spec.surface {
        SurfaceKind::Flat => {
            cetz::line(
                &mut s,
                (-3.0, -0.5),
                (3.0, -0.5),
                Color::Black,
                LineStyle::Solid,
            );
        }
        SurfaceKind::Incline => {
            let l = 4.0;
            let cos = incline.cos();
            let sin = incline.sin();
            let p0 = (-l / 2.0 * cos, -l / 2.0 * sin);
            let p1 = (l / 2.0 * cos, l / 2.0 * sin);
            cetz::line(&mut s, p0, p1, Color::Black, LineStyle::Solid);
            cetz::line(
                &mut s,
                (p0.0 - 0.4, p0.1),
                (p1.0, p0.1),
                Color::Black,
                LineStyle::Solid,
            );
        }
        SurfaceKind::None => {}
    }

    let center = (0.0_f64, 0.0_f64);
    match spec.object {
        ObjectKind::Block | ObjectKind::Beam => {
            let (w, h) = if matches!(spec.object, ObjectKind::Beam) {
                (0.8, 0.2)
            } else {
                (0.5, 0.5)
            };
            let pts = [
                (center.0 - w / 2.0, center.1 - h / 2.0),
                (center.0 + w / 2.0, center.1 - h / 2.0),
                (center.0 + w / 2.0, center.1 + h / 2.0),
                (center.0 - w / 2.0, center.1 + h / 2.0),
            ];
            cetz::polygon(&mut s, &pts, Color::Black, Some(Color::Gray));
        }
        ObjectKind::Sphere => cetz::circle(&mut s, center, 0.3, Color::Black, None),
        ObjectKind::Point => cetz::point(&mut s, center, Color::Black),
    }

    for force in &spec.forces {
        let (theta, label, color) = match force {
            ForceEntry::Gravity(b) => (
                -PI / 2.0,
                b.label.clone().unwrap_or_else(|| "mg".into()),
                Color::Red,
            ),
            ForceEntry::Normal(b) => {
                let t = if matches!(spec.surface, SurfaceKind::Incline) {
                    PI / 2.0 + incline
                } else {
                    PI / 2.0
                };
                (
                    t,
                    b.label.clone().unwrap_or_else(|| "N".into()),
                    Color::Blue,
                )
            }
            ForceEntry::Friction(b) => {
                let t = match b.direction.as_str() {
                    "up_incline" => PI - incline,
                    "down_incline" => -incline,
                    "left" => PI,
                    "right" => 0.0,
                    _ => 0.0,
                };
                (
                    t,
                    b.label.clone().unwrap_or_else(|| "f".into()),
                    Color::Green,
                )
            }
            ForceEntry::Applied(b) => {
                let deg = eval_num(&b.angle, vars).unwrap_or(0.0);
                (
                    deg.to_radians(),
                    b.label.clone().unwrap_or_else(|| "F".into()),
                    Color::Purple,
                )
            }
            ForceEntry::Tension(b) => (
                PI / 2.0,
                b.label.clone().unwrap_or_else(|| "T".into()),
                Color::Orange,
            ),
        };
        let tip = (
            center.0 + ARROW_LEN * theta.cos(),
            center.1 + ARROW_LEN * theta.sin(),
        );
        cetz::arrow(&mut s, center, tip, color);
        let lab_at = (
            center.0 + (ARROW_LEN + 0.25) * theta.cos(),
            center.1 + (ARROW_LEN + 0.25) * theta.sin(),
        );
        cetz::content(&mut s, lab_at, &label);
    }

    Ok(s)
}
