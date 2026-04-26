//! Tiny cetz-markup emitter helpers — escape strings, format f64s.

use std::fmt::Write;

use super::style::{Color, LineStyle};

/// Format an f64 with up to 3 decimals, trimmed.
pub fn n(v: f64) -> String {
    if !v.is_finite() {
        return "0".into();
    }
    let s = format!("{:.3}", v);
    let s = s.trim_end_matches('0').trim_end_matches('.').to_string();
    if s.is_empty() || s == "-0" || s == "-" { "0".into() } else { s }
}

/// Cetz coordinate literal: `(x, y)`.
pub fn pt(x: f64, y: f64) -> String {
    format!("({}, {})", n(x), n(y))
}

/// Escape a label string for embedding in a cetz `content(...)` call.
pub fn label(s: &str) -> String {
    let escaped: String = s
        .chars()
        .flat_map(|c| match c {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            _ => vec![c],
        })
        .collect();
    format!("[{}]", escaped)
}

/// cetz color name from our Color enum.
pub fn color(c: Color) -> &'static str {
    match c {
        Color::Black => "black",
        Color::Blue => "blue",
        Color::Red => "red",
        Color::Green => "green",
        Color::Orange => "orange",
        Color::Purple => "purple",
        Color::Gray => "gray",
        Color::LightBlue => "rgb(\"#aec7e8\")",
        Color::LightGreen => "rgb(\"#98df8a\")",
    }
}

/// `stroke: ...` argument for cetz draw calls.
pub fn stroke(buf: &mut String, c: Color, style: LineStyle) {
    let dash = match style {
        LineStyle::Solid | LineStyle::Thick => "",
        LineStyle::Dashed => ", dash: \"dashed\"",
        LineStyle::Dotted => ", dash: \"dotted\"",
    };
    let width = match style {
        LineStyle::Thick => "1.5pt",
        _ => "0.7pt",
    };
    let _ = write!(buf, "(paint: {}, thickness: {}{})", color(c), width, dash);
}

/// Draw a line from p1 to p2.
pub fn line(buf: &mut String, p1: (f64, f64), p2: (f64, f64), c: Color, s: LineStyle) {
    let _ = write!(buf, "line({}, {}, stroke: ", pt(p1.0, p1.1), pt(p2.0, p2.1));
    stroke(buf, c, s);
    buf.push_str(")\n");
}

/// Draw a circle at center with radius (math units).
pub fn circle(buf: &mut String, center: (f64, f64), r: f64, c: Color, fill: Option<Color>) {
    let fill_arg = match fill {
        Some(f) => format!(", fill: {}", color(f)),
        None => String::new(),
    };
    let _ = write!(
        buf,
        "circle({}, radius: {}, stroke: {}{})\n",
        pt(center.0, center.1),
        n(r),
        color(c),
        fill_arg,
    );
}

/// Draw a closed polygon path.
pub fn polygon(buf: &mut String, pts: &[(f64, f64)], c: Color, fill: Option<Color>) {
    let mut s = String::from("line(");
    for p in pts {
        s.push_str(&pt(p.0, p.1));
        s.push_str(", ");
    }
    s.push_str("close: true, stroke: ");
    s.push_str(color(c));
    if let Some(f) = fill {
        let _ = write!(s, ", fill: {}", color(f));
    }
    s.push_str(")\n");
    buf.push_str(&s);
}

/// Place a text label at a point.
pub fn content(buf: &mut String, at: (f64, f64), text: &str) {
    let _ = write!(buf, "content({}, {})\n", pt(at.0, at.1), label(text));
}

/// Place text with directional anchor relative to the point (north/south/etc).
pub fn content_anchor(buf: &mut String, at: (f64, f64), text: &str, anchor: &str) {
    let _ = write!(
        buf,
        "content({}, {}, anchor: \"{}\")\n",
        pt(at.0, at.1),
        label(text),
        anchor,
    );
}

/// Draw an arrow (line with mark) from p1 to p2.
pub fn arrow(buf: &mut String, p1: (f64, f64), p2: (f64, f64), c: Color) {
    let _ = write!(
        buf,
        "line({}, {}, mark: (end: \">\"), stroke: {})\n",
        pt(p1.0, p1.1),
        pt(p2.0, p2.1),
        color(c),
    );
}

/// Filled point marker (small circle).
pub fn point(buf: &mut String, at: (f64, f64), c: Color) {
    let _ = write!(
        buf,
        "circle({}, radius: 0.08, stroke: {}, fill: {})\n",
        pt(at.0, at.1),
        color(c),
        color(c),
    );
}

/// Open-circle point marker.
pub fn point_open(buf: &mut String, at: (f64, f64), c: Color) {
    let _ = write!(
        buf,
        "circle({}, radius: 0.1, stroke: {}, fill: white)\n",
        pt(at.0, at.1),
        color(c),
    );
}

/// Smooth polyline through sample points.
pub fn polyline(buf: &mut String, pts: &[(f64, f64)], c: Color, s: LineStyle) {
    if pts.len() < 2 { return; }
    let mut out = String::from("line(");
    for p in pts {
        out.push_str(&pt(p.0, p.1));
        out.push_str(", ");
    }
    out.push_str("stroke: ");
    let mut sk = String::new();
    stroke(&mut sk, c, s);
    out.push_str(&sk);
    out.push_str(")\n");
    buf.push_str(&out);
}
