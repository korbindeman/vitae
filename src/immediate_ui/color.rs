use glam::Vec4;

#[derive(Clone, Debug)]
pub struct ColorRGBA(Vec4);

impl ColorRGBA {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        ColorRGBA(Vec4::new(r, g, b, a))
    }

    pub fn to_struct(&self) -> [f32; 4] {
        self.0.to_array()
    }

    pub const WHITE: Self = ColorRGBA(Vec4::splat(1.));
    pub const BLACK: Self = ColorRGBA(Vec4::splat(0.));
    pub const RED: Self = ColorRGBA(Vec4::new(1., 0., 0., 1.));
    pub const GREEN: Self = ColorRGBA(Vec4::new(0., 1., 0., 1.));
    pub const BLUE: Self = ColorRGBA(Vec4::new(0., 0., 1., 1.));
    pub const YELLOW: Self = ColorRGBA(Vec4::new(1., 1., 0., 1.));
    pub const CYAN: Self = ColorRGBA(Vec4::new(0., 1., 1., 1.));
    pub const MAGENTA: Self = ColorRGBA(Vec4::new(1., 0., 1., 1.));
    pub const TRANSPARENT: Self = ColorRGBA(Vec4::splat(0.));
}
