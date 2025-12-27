use crate::color::Color;

#[derive(Clone, Copy, Debug, Default)]
pub struct BorderEdge {
    pub width: f32,
    pub color: Color,
}

impl BorderEdge {
    pub fn new(width: f32, color: Color) -> Self {
        Self { width, color }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Border {
    pub top: BorderEdge,
    pub right: BorderEdge,
    pub bottom: BorderEdge,
    pub left: BorderEdge,
}

impl Border {
    pub fn all(width: f32, color: Color) -> Self {
        let edge = BorderEdge::new(width, color);
        Self {
            top: edge,
            right: edge,
            bottom: edge,
            left: edge,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
    /// When true, radius is computed as 50% of the smaller dimension (full roundness).
    pub full: bool,
}

impl BorderRadius {
    pub fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
            full: false,
        }
    }

    /// Creates a fully rounded border (50% of smaller dimension).
    pub fn full() -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
            full: true,
        }
    }

    /// Returns true if all corners have the same radius.
    pub fn is_uniform(&self) -> bool {
        self.top_left == self.top_right
            && self.top_right == self.bottom_right
            && self.bottom_right == self.bottom_left
    }

    /// Resolve the actual radii given the element dimensions.
    pub fn resolve(&self, width: f32, height: f32) -> (f32, f32, f32, f32) {
        if self.full {
            let r = width.min(height) / 2.0;
            (r, r, r, r)
        } else {
            (
                self.top_left,
                self.top_right,
                self.bottom_right,
                self.bottom_left,
            )
        }
    }
}

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

/// Cross-axis alignment for children (CSS: align-items).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

/// Main-axis distribution of children (CSS: justify-content).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Distribute {
    #[default]
    Start,
    Center,
    End,
    /// Equal space between children, no space at edges.
    Between,
    /// Equal space around each child (half-size space at edges).
    Around,
    /// Equal space between children and at edges.
    Evenly,
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

    pub border: Border,
    pub radius: BorderRadius,

    pub width: Length,
    pub height: Length,
    pub aspect_ratio: Option<f32>,

    pub direction: Direction,
    pub align: Align,
    pub distribute: Distribute,
    pub wrap: bool,
    pub reverse: bool,
    pub gap_x: Length,
    pub gap_y: Length,

    pub font_size: Option<f32>,

    pub position: Position,
    pub top: Option<Length>,
    pub right: Option<Length>,
    pub bottom: Option<Length>,
    pub left: Option<Length>,

    pub opacity: f32,
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
            align: Align::default(),
            distribute: Distribute::default(),
            bg_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
            border: Border::default(),
            radius: BorderRadius::default(),
            wrap: false,
            reverse: false,
            gap_x: Length::Px(0.0),
            gap_y: Length::Px(0.0),
            font_size: None,
            position: Position::default(),
            top: None,
            right: None,
            bottom: None,
            left: None,
            opacity: 1.0,
        }
    }
}
