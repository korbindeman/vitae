use super::color::Color;

#[derive(Clone, Copy, Debug)]
pub enum Length {
    // Percent(f32),
    Px(f32),
    Auto,
}

pub fn px(value: f32) -> Length {
    Length::Px(value)
}

// pub fn percent(value: f32) -> Length {
//     Length::Percent(value)
// }

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

#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeSizes {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

#[derive(Clone, Debug)]
pub struct Style {
    pub margin: EdgeSizes,
    pub padding: EdgeSizes,
    // pub border: EdgeSizes,
    pub bg_color: Color,

    // TODO: min and max width/height
    pub width: Length,
    pub height: Length,

    // layout
    // TODO: align, justify
    pub direction: Direction,
    pub wrap: bool,
    pub reverse: bool, // render children in reverse order
}

impl Default for Style {
    fn default() -> Self {
        Self {
            margin: EdgeSizes::default(),
            padding: EdgeSizes::default(),
            // border: EdgeSizes::default(),
            width: Length::Auto,
            height: Length::Auto,
            direction: Direction::Column,
            bg_color: Color::TRANSPARENT,
            wrap: false,
            reverse: false,
        }
    }
}
