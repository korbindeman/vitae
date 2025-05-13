pub enum DrawCommand {
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
    },
    // … later: Glyph { atlas_uv: […], x,y,w,h, color }
}

pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
