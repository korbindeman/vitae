use glam::Vec4;

#[derive(Clone, Debug)]
pub struct Color(Vec4);

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color(Vec4::new(r, g, b, a))
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color(Vec4::new(
            r as f32 / 255.,
            g as f32 / 255.,
            b as f32 / 255.,
            1.0,
        ))
    }

    pub fn to_array(&self) -> [f32; 4] {
        self.0.to_array()
    }

    pub const WHITE: Self = Color(Vec4::splat(1.));
    pub const BLACK: Self = Color(Vec4::splat(0.));
    pub const GRAY: Self = Color(Vec4::splat(0.5));
    pub const RED: Self = Color(Vec4::new(1., 0., 0., 1.));
    pub const GREEN: Self = Color(Vec4::new(0., 1., 0., 1.));
    pub const BLUE: Self = Color(Vec4::new(0., 0., 1., 1.));
    pub const YELLOW: Self = Color(Vec4::new(1., 1., 0., 1.));
    pub const CYAN: Self = Color(Vec4::new(0., 1., 1., 1.));
    pub const MAGENTA: Self = Color(Vec4::new(1., 0., 1., 1.));
    pub const TRANSPARENT: Self = Color(Vec4::splat(0.));
}
