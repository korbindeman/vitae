use glam::Vec2;
use vitae::{
    App,
    immediate_ui::{
        color::ColorRGBA,
        element::{Direction, ElementHandle, Size},
    },
};

fn main() {
    let root = ElementHandle::new_root(
        Vec2::ZERO,
        ColorRGBA::TRANSPARENT,
        Size::Percent(glam::Vec2::new(100., 100.)),
        Direction::Column,
    );

    for x in 0..8 {
        let row = root.make_child(
            Vec2::ZERO,
            ColorRGBA::TRANSPARENT,
            Size::Percent(glam::Vec2::new(100., 100. / 8.0)),
            Direction::Row,
        );
        for y in 0..8 {
            let color = if y % 2 == x % 2 {
                ColorRGBA::new(0.949, 0.906, 0.906, 1.0)
            } else {
                ColorRGBA::new(0.639, 0.318, 0.306, 1.0)
            };
            let _cell = row.make_child(
                Vec2::ZERO,
                color,
                Size::Percent(glam::Vec2::splat(100. / 8.0)),
                Direction::Row,
            );
        }
    }

    let app = App::new(root);

    pollster::block_on(app.run());
}
