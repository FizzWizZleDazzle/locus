//! Typed `DiagramSpec` enum — replaces `serde_yaml::Value` in `Variant.diagram`.
//!
//! Schema mirrors `docs/DSL_SPEC.md` §11. Discriminator is the YAML `type:`
//! field. Numeric fields use `Num`, a wrapper that deserializes from either
//! YAML number or string so callers can write `radius: 5` or `radius: r`
//! interchangeably. Resolution happens at render time via
//! `super::eval::eval_num`.

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer};

use super::style::{Color, LineStyle, PointStyle};

/// Numeric-or-expression field. Accepts YAML int, float, bool, or string.
#[derive(Debug, Clone, Default)]
pub struct Num(pub String);

impl Num {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for Num {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Num {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = serde_yaml::Value::deserialize(d)?;
        let s = match v {
            serde_yaml::Value::String(s) => s,
            serde_yaml::Value::Number(n) => n.to_string(),
            serde_yaml::Value::Bool(b) => b.to_string(),
            other => {
                return Err(serde::de::Error::custom(format!(
                    "expected number or expression string, got {other:?}"
                )));
            }
        };
        Ok(Num(s))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum DiagramSpec {
    NumberLine(NumberLine),
    CoordinatePlane(CoordinatePlane),
    Triangle(Triangle),
    Circle(Circle),
    Polygon(Polygon),
    FunctionGraph(FunctionGraph),
    ForceDiagram(ForceDiagram),
    Field(Field),
    Circuit(Circuit),
}

// ---------- Number line (§11.5) ----------

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumberLine {
    pub range: [Num; 2],
    #[serde(default)]
    pub elements: Vec<NumberLineElement>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum NumberLineElement {
    Point(NlPoint),
    Segment(NlSegment),
    Arrow(NlArrow),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NlPoint {
    pub at: Num,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub style: PointStyle,
    #[serde(default)]
    pub color: Color,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NlSegment {
    pub from: Num,
    pub to: Num,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub style: LineStyle,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NlArrow {
    pub from: Num,
    /// Direction is optional when `to:` is set — sign of `to - from` then
    /// determines orientation. Required when `to:` is absent.
    #[serde(default)]
    pub direction: Option<ArrowDirection>,
    #[serde(default)]
    pub color: Color,
    /// When set, the arrow ends at this position instead of the diagram edge.
    #[serde(default)]
    pub to: Option<Num>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArrowDirection {
    Left,
    Right,
}

// ---------- Coordinate plane (§11.1) ----------

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinatePlane {
    pub x_range: [Num; 2],
    pub y_range: [Num; 2],
    #[serde(default)]
    pub grid: bool,
    #[serde(default)]
    pub elements: Vec<CpElement>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum CpElement {
    Line(CpLine),
    Point(CpPoint),
    Shade(CpShade),
    Asymptote(CpAsymptote),
    Arrow(CpArrow),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CpLine {
    pub slope: Num,
    pub intercept: Num,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub style: LineStyle,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CpPoint {
    pub x: Num,
    pub y: Num,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub style: PointStyle,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CpShade {
    /// Names of two `line` elements (by their `label`); shade between.
    pub between: [String; 2],
    pub from: Num,
    pub to: Num,
    #[serde(default)]
    pub color: Color,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CpAsymptote {
    /// Vertical asymptote at `x = ...` if `x` set; horizontal at `y = ...` if `y` set.
    #[serde(default)]
    pub x: Option<Num>,
    #[serde(default)]
    pub y: Option<Num>,
    #[serde(default)]
    pub style: LineStyle,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CpArrow {
    pub from: [Num; 2],
    pub to: [Num; 2],
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub color: Color,
}

// ---------- Triangle (§11.2) ----------

/// Side metadata: bare `Num` shorthand or `{length, label, style}` form.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SideValue {
    Length(Num),
    Meta {
        #[serde(default)]
        length: Option<Num>,
        #[serde(default)]
        label: Option<String>,
        #[serde(default)]
        style: LineStyle,
    },
}

impl SideValue {
    pub fn length(&self) -> Option<&str> {
        match self {
            SideValue::Length(n) => Some(n.as_str()),
            SideValue::Meta { length, .. } => length.as_deref(),
        }
    }
    pub fn label(&self) -> Option<&str> {
        match self {
            SideValue::Length(_) => None,
            SideValue::Meta { label, .. } => label.as_deref(),
        }
    }
    pub fn style(&self) -> LineStyle {
        match self {
            SideValue::Length(_) => LineStyle::Solid,
            SideValue::Meta { style, .. } => *style,
        }
    }
}

/// Angle value: numeric expression or marker like `"right_angle"`.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AngleValue {
    Numeric(Num),
    Marker(String),
}

impl AngleValue {
    pub fn as_num(&self) -> Option<&str> {
        match self {
            AngleValue::Numeric(n) => Some(n.as_str()),
            AngleValue::Marker(_) => None,
        }
    }
    pub fn marker(&self) -> Option<&str> {
        match self {
            AngleValue::Marker(s) => Some(s.as_str()),
            AngleValue::Numeric(_) => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Triangle {
    pub vertices: [String; 3],
    #[serde(default)]
    pub sides: BTreeMap<String, SideValue>,
    #[serde(default)]
    pub angles: BTreeMap<String, AngleValue>,
    #[serde(default)]
    pub marks: BTreeMap<String, String>,
    #[serde(default)]
    pub right_angle: Option<String>,
    /// Tolerated; renderer ignores additional element decorations like `ray:`.
    #[serde(default)]
    pub elements: Vec<serde_yaml::Value>,
}

// ---------- Circle (§11.3) ----------

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Circle {
    pub center: String,
    /// Radius is informational; defaults to `1` when absent (geometry of the
    /// rendered picture is independent of magnitude).
    #[serde(default)]
    pub radius: Option<Num>,
    #[serde(default)]
    pub elements: Vec<CircleElement>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum CircleElement {
    Chord(ChordSpec),
    Arc(ArcSpec),
    Radius(RadiusSpec),
    Tangent(TangentSpec),
    CentralAngle(CentralAngleSpec),
    InscribedAngle(InscribedAngleSpec),
    Point(CirclePoint),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChordSpec {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub color: Color,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArcSpec {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RadiusSpec {
    pub to: String,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TangentSpec {
    pub at: String,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentralAngleSpec {
    pub vertex: String,
    pub sides: [String; 2],
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InscribedAngleSpec {
    pub vertex: String,
    pub sides: [String; 2],
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CirclePoint {
    pub name: String,
    /// Angle in degrees from the positive x-axis.
    pub angle: Num,
    #[serde(default)]
    pub label: Option<String>,
}

// ---------- Polygon (§11.4) ----------

/// Vertex spec: bare name (positions auto-computed) or explicit `{x, y}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Vertex {
    Named(String),
    Coord {
        #[serde(default)]
        name: Option<String>,
        x: Num,
        y: Num,
    },
}

impl Vertex {
    pub fn name(&self, idx: usize) -> String {
        match self {
            Vertex::Named(n) => n.clone(),
            Vertex::Coord { name: Some(n), .. } => n.clone(),
            Vertex::Coord { name: None, .. } => format!("V{idx}"),
        }
    }
    pub fn coord(&self) -> Option<(&str, &str)> {
        match self {
            Vertex::Coord { x, y, .. } => Some((x.as_str(), y.as_str())),
            Vertex::Named(_) => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
    #[serde(default)]
    pub sides: BTreeMap<String, SideValue>,
    #[serde(default)]
    pub angles: BTreeMap<String, AngleValue>,
    #[serde(default)]
    pub parallel: Vec<[String; 2]>,
    #[serde(default)]
    pub labels: BTreeMap<String, String>,
}

// ---------- Function graph (§11.6) ----------

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionGraph {
    pub functions: Vec<FunctionEntry>,
    pub x_range: [Num; 2],
    pub y_range: [Num; 2],
    #[serde(default)]
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionEntry {
    pub expr: String,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Feature {
    Zero(FeatureBody),
    Maximum(FeatureBody),
    Minimum(FeatureBody),
    Inflection(FeatureBody),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureBody {
    pub of: String,
    #[serde(default)]
    pub label: bool,
    #[serde(default)]
    pub style: PointStyle,
}

// ---------- Force diagram (§11.7) ----------

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForceDiagram {
    pub object: ObjectKind,
    pub surface: SurfaceKind,
    #[serde(default)]
    pub incline_angle: Option<Num>,
    pub forces: Vec<ForceEntry>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectKind {
    Block,
    Sphere,
    Point,
    Beam,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    Flat,
    Incline,
    None,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum ForceEntry {
    Gravity(ForceBody),
    Normal(ForceBody),
    Friction(FrictionBody),
    Applied(AppliedBody),
    Tension(ForceBody),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForceBody {
    #[serde(default)]
    pub magnitude: Option<Num>,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrictionBody {
    /// `up_incline` | `down_incline` | `left` | `right`.
    pub direction: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub magnitude: Option<Num>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppliedBody {
    pub angle: Num,
    #[serde(default)]
    pub magnitude: Option<Num>,
    #[serde(default)]
    pub label: Option<String>,
}

// ---------- Field lines (§11.8) ----------

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub field_type: FieldKind,
    pub sources: Vec<FieldSource>,
    #[serde(default)]
    pub show_lines: bool,
    #[serde(default)]
    pub show_equipotential: bool,
    pub region: [Num; 4],
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldKind {
    Electric,
    Magnetic,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum FieldSource {
    Charge(ChargeBody),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChargeBody {
    pub value: Num,
    pub position: [Num; 2],
    #[serde(default)]
    pub label: Option<String>,
}

// ---------- Circuit (§11.9) — accepted but renderer errors ----------

/// Circuit diagrams are accepted at parse time so existing YAMLs validate,
/// but the renderer returns an error. circuitikz pipeline is future work.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Circuit {
    #[serde(default)]
    pub elements: Vec<serde_yaml::Value>,
    #[serde(default)]
    pub layout: Option<String>,
}
