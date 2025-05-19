use vitae::{
    App,
    immediate_ui::{color::ColorRGBA, element::Direction, elements::div::div},
};

fn main() {
    fn checkerboard(x: i32, y: i32) -> ColorRGBA {
        let light_square = ColorRGBA::new(0.949, 0.906, 0.906, 1.0);
        let dark_square = ColorRGBA::new(0.639, 0.318, 0.306, 1.0);

        // bitwise sum‐parity pick, tbh idk how this really works
        if ((x + y) & 1) == 0 {
            light_square
        } else {
            dark_square
        }
    }

    let root = div().direction(Direction::Column).children(
        (0..8)
            .map(|x| {
                div().height(100. / 8.0).children(
                    (0..8)
                        .map(|y| {
                            let color = checkerboard(x, y);
                            div().background(color).width(100. / 8.0)
                        })
                        .collect(),
                )
            })
            .collect(),
    );

    let app = App::new(root);

    pollster::block_on(app.run());
}
