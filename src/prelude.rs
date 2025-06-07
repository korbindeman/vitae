pub use crate::App;
pub use crate::core::color::*;
pub use crate::core::elements::div::div;
pub use crate::core::style::*;

// TODO: these should be somewhere else, probably in a theme module
// SIZES
pub const FULL: Length = Length::Percent(100.);
pub const HALF: Length = Length::Percent(50.);

// SPACING
pub const SM: Length = Length::Px(8.);
pub const MD: Length = Length::Px(16.);
pub const LG: Length = Length::Px(32.);

// COLORS
pub const WHITE: Color = Color::WHITE;
pub const BLACK: Color = Color::BLACK;
pub const GRAY: Color = Color::GRAY;
pub const RED: Color = Color::RED;
pub const GREEN: Color = Color::GREEN;
pub const BLUE: Color = Color::BLUE;
pub const YELLOW: Color = Color::YELLOW;
pub const CYAN: Color = Color::CYAN;
pub const MAGENTA: Color = Color::MAGENTA;
pub const TRANSPARENT: Color = Color::TRANSPARENT;
