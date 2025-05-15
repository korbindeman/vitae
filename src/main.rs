use glam::Vec2;
use vitae::{
    App,
    immediate_ui::{
        colors::{RED, WHITE},
        elements::{ElementHandle, Size},
    },
};

fn main() {
    let root = ElementHandle::new_root(
        Vec2::ZERO,
        WHITE,
        Size::Percentage(glam::Vec2::new(100., 30.)),
    );
    let _child = root.make_child(Vec2::ZERO, RED, Size::Percentage(glam::Vec2::new(40., 20.)));

    let app = App::new(root);

    pollster::block_on(app.run());
}
