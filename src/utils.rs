use vitae::core::color::Color;

pub fn checkerboard(x: i32, y: i32) -> Color {
    let light_square = Color::rgb(242, 229, 229);
    let dark_square = Color::rgb(163, 82, 76);

    // bitwise sum‐parity pick, tbh idk how this really works
    if ((x + y) & 1) == 0 {
        light_square
    } else {
        dark_square
    }
}
