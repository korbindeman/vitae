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
