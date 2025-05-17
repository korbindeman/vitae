use glam::Vec4;

use super::color::ColorRGBA;

#[derive(Clone, Copy, Debug)]
pub enum Length {
    Percent(f32),
    Scale(f32),
    Px(f32),
}

pub struct Length2 {
    pub x: Length,
    pub y: Length,
}

#[derive(Clone, Debug)]
pub struct Style {
    pub margin: Vec4,
    pub padding: Vec4,
    pub bg_color: ColorRGBA,
}

impl Style {
    pub fn from_bg_color(bg_color: ColorRGBA) -> Self {
        Self {
            margin: Vec4::splat(0.0),
            padding: Vec4::splat(0.0),
            bg_color,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            margin: Vec4::splat(0.0),
            padding: Vec4::splat(0.0),
            bg_color: ColorRGBA::WHITE,
        }
    }
}
