//! Color, line-style, point-style enums per DSL_SPEC.md §11.10.

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default]
    Black,
    Blue,
    Red,
    Green,
    Orange,
    Purple,
    Gray,
    LightBlue,
    LightGreen,
}

impl Color {
    /// CSS color value emitted into the SVG `stroke`/`fill` attributes.
    pub fn css(self) -> &'static str {
        match self {
            Color::Black => "currentColor",
            Color::Blue => "#1f77b4",
            Color::Red => "#d62728",
            Color::Green => "#2ca02c",
            Color::Orange => "#ff7f0e",
            Color::Purple => "#9467bd",
            Color::Gray => "#7f7f7f",
            Color::LightBlue => "#aec7e8",
            Color::LightGreen => "#98df8a",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LineStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
    Thick,
}

impl LineStyle {
    /// `stroke-dasharray` value, or `None` for solid lines.
    pub fn dash(self) -> Option<&'static str> {
        match self {
            LineStyle::Solid | LineStyle::Thick => None,
            LineStyle::Dashed => Some("6,4"),
            LineStyle::Dotted => Some("2,3"),
        }
    }

    pub fn stroke_width(self) -> f64 {
        if matches!(self, LineStyle::Thick) {
            2.5
        } else {
            1.5
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PointStyle {
    #[default]
    Filled,
    Open,
    Cross,
    Square,
}
