//! Diagram render round-trip tests.
//!
//! For each implemented diagram type, parse a representative YAML, generate a
//! problem, and assert `question_image_url`:
//! - is non-empty
//! - starts with the dictionary-compression prefix `s1:`
//! - decompresses to valid SVG containing `<svg ...>` and `</svg>`

use locus_common::svg_compress::decompress_svg;
use locus_dsl::{generate_random, parse};

fn render_question_image_url(yaml: &str) -> String {
    let spec = parse(yaml).expect("parse");
    let out = generate_random(&spec).expect("generate");
    out.question_image_url
}

fn assert_compressed_svg(image: &str) {
    assert!(!image.is_empty(), "expected non-empty question_image_url");
    assert!(
        image.starts_with("s1:"),
        "expected compression prefix, got: {}",
        &image[..image.len().min(40)]
    );
    let svg = decompress_svg(image);
    assert!(
        svg.contains("<svg "),
        "decompressed not SVG: {}",
        &svg[..svg.len().min(80)]
    );
    assert!(svg.contains("</svg>"), "decompressed missing close tag");
}

#[test]
fn number_line_renders() {
    let yaml = r#"
topic: algebra1/test
difficulty: easy
variants:
  - name: default
    variables:
      a: integer(1, 5)
    question: 'point at {a}'
    answer: a
    diagram:
      type: number_line
      range: [-5, 5]
      elements:
        - point: {at: a, style: filled}
        - arrow: {from: a, direction: right, color: blue}
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn coordinate_plane_renders() {
    let yaml = r#"
topic: algebra1/test
difficulty: easy
variants:
  - name: default
    variables:
      m: integer(1, 3)
      b: integer(-2, 2)
    question: 'graph it'
    answer: m
    diagram:
      type: coordinate_plane
      x_range: [-5, 5]
      y_range: [-8, 8]
      grid: true
      elements:
        - line: {slope: m, intercept: b, color: blue, label: y}
        - point: {x: 0, y: b, label: P}
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn circle_renders_with_central_and_inscribed_angles() {
    let yaml = r#"
topic: geometry/test
difficulty: easy
variants:
  - name: default
    variables:
      central: choice(40, 60, 80)
      half_central: central / 2
    question: 'inscribed?'
    answer: half_central
    diagram:
      type: circle
      center: O
      radius: r
      elements:
        - central_angle: {vertex: O, sides: [A, B], label: central}
        - inscribed_angle: {vertex: C, sides: [A, B], label: half_central}
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn polygon_renders_trapezoid() {
    let yaml = r#"
topic: geometry/test
difficulty: medium
variants:
  - name: default
    variables:
      b1: integer(3, 8)
      b2: integer(3, 8)
      ht: integer(2, 6)
    constraints:
      - b1 != b2
    question: 'area?'
    answer: b1
    diagram:
      type: polygon
      vertices: [A, B, C, D]
      sides:
        AB: {label: b1}
        CD: {label: b2}
        BC: {label: ht, style: dashed}
      angles:
        B: right_angle
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn triangle_renders_with_angles() {
    let yaml = r#"
topic: geometry/test
difficulty: easy
variants:
  - name: default
    variables:
      angle_a: integer(20, 70)
      angle_b: integer(20, 70)
      angle_c: 180 - angle_a - angle_b
    constraints:
      - angle_c > 10
    question: 'find angle'
    answer: angle_c
    diagram:
      type: triangle
      vertices: [A, B, C]
      angles: {A: angle_a, B: angle_b, C: angle_c}
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn triangle_with_right_angle_and_sides() {
    let yaml = r#"
topic: geometry/test
difficulty: easy
variants:
  - name: default
    variables:
      hyp: choice(4, 6, 8)
      angle_deg: 30
      answer: hyp / 2
    question: 'opposite?'
    answer: answer
    diagram:
      type: triangle
      vertices: [A, B, C]
      sides:
        AB: hyp
        BC: answer
      angles:
        A: angle_deg
      right_angle: B
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn function_graph_renders_parabola() {
    let yaml = r#"
topic: precalc/test
difficulty: easy
variants:
  - name: default
    variables:
      a: integer(1, 3)
    question: 'graph it'
    answer: a
    diagram:
      type: function_graph
      functions:
        - {expr: "x^2 - 4", color: blue, label: f}
      x_range: [-3, 3]
      y_range: [-5, 5]
      features:
        - zero: {of: f, label: true}
        - minimum: {of: f, label: true}
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn force_diagram_renders_block_on_incline() {
    let yaml = r#"
topic: physics_c_mech/test
difficulty: medium
variants:
  - name: default
    variables:
      theta: integer(20, 40)
      m: integer(2, 8)
    question: 'forces?'
    answer: m
    diagram:
      type: force_diagram
      object: block
      surface: incline
      incline_angle: theta
      forces:
        - gravity: {magnitude: mg, label: "mg"}
        - normal: {label: "N"}
        - friction: {direction: up_incline, label: "f"}
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn field_diagram_renders_dipole() {
    let yaml = r#"
topic: physics_c_em/test
difficulty: medium
variants:
  - name: default
    variables:
      q: integer(1, 3)
    question: 'field?'
    answer: q
    diagram:
      type: field
      field_type: electric
      sources:
        - charge: {value: q, position: [-2, 0], label: "+q"}
        - charge: {value: -q, position: [2, 0], label: "-q"}
      show_lines: true
      region: [-5, 5, -5, 5]
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}

#[test]
fn number_line_with_to_field_tolerated() {
    let yaml = r#"
topic: algebra1/test
difficulty: easy
variants:
  - name: default
    variables:
      a: integer(1, 3)
      b: integer(4, 6)
    question: 'between {a} and {b}'
    answer: a
    diagram:
      type: number_line
      range: [-6, 7]
      elements:
        - point: {at: a, style: filled}
        - point: {at: b, style: filled}
        - arrow: {from: a, to: b, direction: right, color: blue}
"#;
    assert_compressed_svg(&render_question_image_url(yaml));
}
