pub use crate::{use_signal, App, Signal};
pub use vitae_core::{
    div, img, pc, portal, px, svg, text, Align, Color, Direction, Distribute, ElementBuilder,
    Length, Svg, Texture,
};
pub use vitae_render::{load_svg, load_texture};

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
