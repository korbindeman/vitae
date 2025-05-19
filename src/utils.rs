use vitae::immediate_ui::color::ColorRGBA;

pub fn checkerboard(x: i32, y: i32) -> ColorRGBA {
    let light_square = ColorRGBA::rgb(0.95, 0.9, 0.9);
    let dark_square = ColorRGBA::rgb(0.64, 0.32, 0.3);

    // bitwise sum‐parity pick, tbh idk how this really works
    if ((x + y) & 1) == 0 {
        light_square
    } else {
        dark_square
    }
}
