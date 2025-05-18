use vitae::{
    App,
    immediate_ui::{
        color::ColorRGBA,
        element::{Direction, Size},
        elements::div::div,
    },
};

fn main() {
    let root = div(ColorRGBA::TRANSPARENT, Size::FULL, Direction::Column);

    for x in 0..8 {
        let row = root.child(div(
            ColorRGBA::TRANSPARENT,
            Size::percent(100., 100. / 8.0),
            Direction::Row,
        ));
        for y in 0..8 {
            let color = if y % 2 == x % 2 {
                ColorRGBA::new(0.949, 0.906, 0.906, 1.0)
            } else {
                ColorRGBA::new(0.639, 0.318, 0.306, 1.0)
            };
            let _cell = row.child(div(
                color,
                Size::percent(100. / 8.0, 100. / 8.0),
                Direction::Row,
            ));
        }
    }

    let app = App::new(root);

    pollster::block_on(app.run());
}
