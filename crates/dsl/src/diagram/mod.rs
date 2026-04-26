//! Diagram subsystem — Typst+cetz declarative SVG generation.
//!
//! See `docs/DSL_SPEC.md` §11. Each renderer translates a `DiagramSpec`
//! variant into cetz markup; `compile::compile` runs Typst (with bundled
//! cetz + oxifmt packages) to produce SVG; the result is fed through
//! `locus_common::svg_compress::compress_svg` and stored in
//! `ProblemOutput.question_image_url`. The frontend's `decompress_svg`
//! (`crates/frontend/src/components/problem_card.rs:30`) inverts the same
//! dictionary on display.

pub mod cetz;
pub mod compile;
pub mod eval;
pub mod spec;
pub mod style;
pub mod world;

mod circle;
mod coordinate_plane;
mod field;
mod force_diagram;
mod function_graph;
mod number_line;
mod polygon;
mod triangle;

use locus_common::svg_compress::compress_svg;

use crate::error::DslError;
use crate::resolver::VarMap;

use spec::DiagramSpec;

/// Render a `DiagramSpec` to a compressed SVG string. Each renderer emits
/// cetz markup; `compile::compile` runs Typst to produce SVG; the result is
/// dictionary-compressed for storage.
///
/// Per-type cetz canvas `length` (cetz unit -> physical size) is chosen so
/// labels don't crowd shapes. Triangles/circles/polygons work in tight
/// math units so they need a longer length than e.g. number lines.
pub fn render(spec: &DiagramSpec, vars: &VarMap) -> Result<String, DslError> {
    let (canvas_body, length) = match spec {
        DiagramSpec::NumberLine(d) => (number_line::render(d, vars)?, "0.7cm"),
        DiagramSpec::CoordinatePlane(d) => (coordinate_plane::render(d, vars)?, "0.5cm"),
        DiagramSpec::Triangle(d) => (triangle::render(d, vars)?, "0.7cm"),
        DiagramSpec::Circle(d) => (circle::render(d, vars)?, "2.4cm"),
        DiagramSpec::Polygon(d) => (polygon::render(d, vars)?, "0.8cm"),
        DiagramSpec::FunctionGraph(d) => (function_graph::render(d, vars)?, "0.5cm"),
        DiagramSpec::ForceDiagram(d) => (force_diagram::render(d, vars)?, "0.9cm"),
        DiagramSpec::Field(d) => (field::render(d, vars)?, "0.5cm"),
        DiagramSpec::Circuit(_) => {
            return Err(DslError::Evaluation(
                "circuit diagrams not yet implemented (circuitikz pipeline pending)".into(),
            ));
        }
    };
    let typst_src = compile::wrap_cetz_with_length(&canvas_body, length);
    let raw = compile::compile(typst_src)?;
    Ok(compress_svg(&raw))
}
