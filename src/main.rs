use vitae::{
    App,
    immediate_ui::{color::ColorRGBA, element::Direction, elements::div::div},
};

fn main() {
    fn checkerboard(x: i32, y: i32) -> ColorRGBA {
        let light_square = ColorRGBA::rgb(0.95, 0.9, 0.9);
        let dark_square = ColorRGBA::rgb(0.64, 0.32, 0.3);

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
