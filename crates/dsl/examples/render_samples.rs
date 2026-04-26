//! Render one sample of each diagram type to /tmp/diagram_examples/<name>.svg.
//!
//! Run: `cargo run -p locus-dsl --example render_samples`

use std::fs;

use locus_common::svg_compress::decompress_svg;
use locus_dsl::{generate_random, parse};

const SAMPLES: &[(&str, &str)] = &[
    ("number_line", r#"
topic: algebra1/test
difficulty: easy
variants:
  - name: default
    variables:
      a: integer(2, 4)
      b: integer(-3, -1)
    question: 'q'
    answer: a
    diagram:
      type: number_line
      range: [-5, 5]
      elements:
        - point: {at: a, style: filled, label: a}
        - point: {at: b, style: open, label: b}
        - segment: {from: b, to: a, color: blue}
        - arrow: {from: a, direction: right, color: red}
"#),
    ("coordinate_plane", r#"
topic: algebra1/test
difficulty: easy
variants:
  - name: default
    variables:
      m: choice(2)
      b: choice(-1)
    question: 'q'
    answer: m
    diagram:
      type: coordinate_plane
      x_range: [-5, 5]
      y_range: [-8, 8]
      grid: true
      elements:
        - line: {slope: m, intercept: b, color: blue, label: y}
        - point: {x: 0, y: b, label: P}
        - asymptote: {x: 3, style: dashed}
"#),
    ("circle", r#"
topic: geometry/test
difficulty: easy
variants:
  - name: default
    variables:
      central: choice(60)
      half_central: central / 2
    question: 'q'
    answer: half_central
    diagram:
      type: circle
      center: O
      radius: r
      elements:
        - central_angle: {vertex: O, sides: [A, B], label: central}
        - inscribed_angle: {vertex: C, sides: [A, B], label: half_central}
"#),
    ("polygon_trapezoid", r#"
topic: geometry/test
difficulty: medium
variants:
  - name: default
    variables:
      b1: choice(6)
      b2: choice(4)
      ht: choice(3)
    question: 'q'
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
"#),
    ("polygon_l_shape", r#"
topic: geometry/test
difficulty: medium
variants:
  - name: default
    variables:
      a: choice(5)
      b: choice(4)
      c: choice(2)
      d: choice(2)
      width: a + c
      height: b + d
    question: 'q'
    answer: a
    diagram:
      type: polygon
      vertices:
        - {x: 0, y: 0}
        - {x: width, y: 0}
        - {x: width, y: b}
        - {x: a, y: b}
        - {x: a, y: height}
        - {x: 0, y: height}
"#),
    ("triangle", r#"
topic: geometry/test
difficulty: easy
variants:
  - name: default
    variables:
      angle_a: choice(40)
      angle_b: choice(70)
      angle_c: 180 - angle_a - angle_b
    question: 'q'
    answer: angle_c
    diagram:
      type: triangle
      vertices: [A, B, C]
      angles: {A: angle_a, B: angle_b, C: angle_c}
"#),
    ("triangle_right", r#"
topic: geometry/test
difficulty: easy
variants:
  - name: default
    variables:
      hyp: choice(8)
      angle_deg: choice(30)
      ans: hyp / 2
    question: 'q'
    answer: ans
    diagram:
      type: triangle
      vertices: [A, B, C]
      sides:
        AB: hyp
        BC: ans
      angles:
        A: angle_deg
      right_angle: B
"#),
    ("function_graph", r#"
topic: precalc/test
difficulty: easy
variants:
  - name: default
    variables:
      a: choice(1)
    question: 'q'
    answer: a
    diagram:
      type: function_graph
      functions:
        - {expr: "x^2 - 4", color: blue, label: f}
        - {expr: "sin(x)", color: red, label: g}
      x_range: [-3, 3]
      y_range: [-5, 5]
      features:
        - zero: {of: f, label: true}
        - minimum: {of: f, label: true}
"#),
    ("force_diagram", r#"
topic: physics_c_mech/test
difficulty: medium
variants:
  - name: default
    variables:
      theta: choice(30)
    question: 'q'
    answer: theta
    diagram:
      type: force_diagram
      object: block
      surface: incline
      incline_angle: theta
      forces:
        - gravity: {label: "mg"}
        - normal: {label: "N"}
        - friction: {direction: up_incline, label: "f"}
        - applied: {angle: 30, label: "F"}
"#),
    ("field_dipole", r#"
topic: physics_c_em/test
difficulty: medium
variants:
  - name: default
    variables:
      q: choice(2)
    question: 'q'
    answer: q
    diagram:
      type: field
      field_type: electric
      sources:
        - charge: {value: q, position: [-2, 0], label: "+q"}
        - charge: {value: -q, position: [2, 0], label: "-q"}
      show_lines: true
      region: [-5, 5, -5, 5]
"#),
];

fn main() {
    let out_dir = "/tmp/diagram_examples";
    fs::create_dir_all(out_dir).unwrap();
    for (name, yaml) in SAMPLES {
        let spec = parse(yaml).unwrap_or_else(|e| panic!("{name}: parse: {e}"));
        let out = generate_random(&spec).unwrap_or_else(|e| panic!("{name}: gen: {e}"));
        let svg = decompress_svg(&out.question_image_url);
        let path = format!("{out_dir}/{name}.svg");
        fs::write(&path, &svg).unwrap();
        println!("{name}: {} bytes raw, {} bytes compressed", svg.len(), out.question_image_url.len());
    }
    println!("\nWrote samples to {out_dir}/");
}
