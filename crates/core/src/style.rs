use crate::color::Color;

#[derive(Clone, Copy, Debug)]
pub enum Length {
    Percent(f32),
    Px(f32),
    Auto,
}

impl Length {
    pub fn as_px(&self) -> f32 {
        match self {
            Length::Px(px) => *px,
            _ => 0.0,
        }
    }
}

/// Create a length in pixels.
pub fn px(value: f32) -> Length {
    Length::Px(value)
}

/// Create a length in percentage.
pub fn pc(value: f32) -> Length {
    Length::Percent(value)
}

impl Default for Length {
    fn default() -> Self {
        Length::Auto
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Direction {
    Column,
    Row,
}

#[derive(Clone, Debug, PartialEq, Copy, Default)]
pub enum Position {
    #[default]
    Relative,
    Absolute,
    /// Positioned relative to the viewport, rendered on top of everything.
    Portal,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeSizes {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

impl EdgeSizes {
    pub fn new(top: Length, right: Length, bottom: Length, left: Length) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn splat(value: Length) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Style {
    pub margin: EdgeSizes,
    pub padding: EdgeSizes,
    pub bg_color: Color,
    pub text_color: Color,

    pub width: Length,
    pub height: Length,
    pub aspect_ratio: Option<f32>,

    pub direction: Direction,
    pub wrap: bool,
    pub reverse: bool,

    pub font_size: Option<f32>,

    pub position: Position,
    pub top: Option<Length>,
    pub right: Option<Length>,
    pub bottom: Option<Length>,
    pub left: Option<Length>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            margin: EdgeSizes::default(),
            padding: EdgeSizes::default(),
            width: Length::Auto,
            height: Length::Auto,
            aspect_ratio: None,
            direction: Direction::Column,
            bg_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
            wrap: false,
            reverse: false,
            font_size: None,
            position: Position::default(),
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }
}
