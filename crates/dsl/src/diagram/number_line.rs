//! Number line renderer (§11.5) — emits cetz markup.

use std::fmt::Write;

use crate::error::DslError;
use crate::resolver::VarMap;

use super::cetz;
use super::eval::eval_num;
use super::spec::{ArrowDirection, NumberLine, NumberLineElement};
use super::style::{Color, LineStyle, PointStyle};

pub fn render(spec: &NumberLine, vars: &VarMap) -> Result<String, DslError> {
    let lo = eval_num(&spec.range[0], vars)?;
    let hi = eval_num(&spec.range[1], vars)?;
    if hi <= lo {
        return Err(DslError::Evaluation(format!(
            "number_line range must be ascending: [{lo}, {hi}]"
        )));
    }

    let mut s = String::new();
    cetz::line(&mut s, (lo, 0.0), (hi, 0.0), Color::Black, LineStyle::Solid);

    let lo_i = lo.ceil() as i64;
    let hi_i = hi.floor() as i64;
    for i in lo_i..=hi_i {
        let x = i as f64;
        cetz::line(&mut s, (x, -0.1), (x, 0.1), Color::Black, LineStyle::Solid);
        cetz::content_anchor(&mut s, (x, -0.4), &i.to_string(), "north");
    }

    for el in &spec.elements {
        match el {
            NumberLineElement::Point(p) => {
                let x = eval_num(&p.at, vars)?;
                match p.style {
                    PointStyle::Open => cetz::point_open(&mut s, (x, 0.0), p.color),
                    _ => cetz::point(&mut s, (x, 0.0), p.color),
                }
                if let Some(label) = &p.label {
                    cetz::content_anchor(&mut s, (x, 0.4), label, "south");
                }
            }
            NumberLineElement::Segment(seg) => {
                let x1 = eval_num(&seg.from, vars)?;
                let x2 = eval_num(&seg.to, vars)?;
                let _ = write!(
                    s,
                    "line({}, {}, stroke: (paint: {}, thickness: 2pt))\n",
                    cetz::pt(x1, 0.0), cetz::pt(x2, 0.0), cetz::color(seg.color),
                );
            }
            NumberLineElement::Arrow(a) => {
                let x_from = eval_num(&a.from, vars)?;
                let x_to = if let Some(to_expr) = &a.to {
                    eval_num(to_expr, vars)?
                } else {
                    let dir = a.direction.unwrap_or(ArrowDirection::Right);
                    match dir {
                        ArrowDirection::Right => hi + 0.5,
                        ArrowDirection::Left => lo - 0.5,
                    }
                };
                cetz::arrow(&mut s, (x_from, 0.0), (x_to, 0.0), a.color);
            }
        }
    }

    Ok(s)
}
