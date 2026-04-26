//! Compile a Typst markup string to SVG via `typst::compile` + `typst-svg`.

use typst::layout::{Abs, PagedDocument};

use crate::error::DslError;

use super::world::InMemoryWorld;

/// Wrap a cetz `canvas` body in a self-contained Typst document with the cetz
/// import already pulled in. Page size is auto from content.
pub fn wrap_cetz(canvas_body: &str) -> String {
    wrap_cetz_with_length(canvas_body, "1cm")
}

/// Wrap with explicit canvas length (one cetz unit = `length`). Use for
/// renderers whose math-unit content is small (triangles, circles, small
/// polygons) so labels don't crowd shapes.
pub fn wrap_cetz_with_length(canvas_body: &str, length: &str) -> String {
    format!(
        "#import \"@preview/cetz:0.5.0\"\n\
         #set page(width: auto, height: auto, margin: 8pt)\n\
         #set text(size: 9pt)\n\
         #cetz.canvas(length: {length}, {{\n\
           import cetz.draw: *\n\
           import cetz.angle\n\
           {body}\n\
         }})\n",
        length = length,
        body = canvas_body,
    )
}

/// Compile a complete Typst source to SVG (one merged page).
pub fn compile(typst_src: String) -> Result<String, DslError> {
    let world = InMemoryWorld::new(typst_src);
    let warned = typst::compile::<PagedDocument>(&world);
    let doc = warned.output.map_err(|errors| {
        let msg = errors
            .iter()
            .map(|e| format!("{}", e.message))
            .collect::<Vec<_>>()
            .join("; ");
        DslError::Evaluation(format!("typst compile failed: {msg}"))
    })?;
    Ok(typst_svg::svg_merged(&doc, Abs::zero()))
}
