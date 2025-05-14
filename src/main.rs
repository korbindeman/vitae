use vitae::{
    App,
    immediate_ui::{
        colors::{RED, WHITE},
        elements::{Element, Size},
    },
};

fn main() {
    let child_element = Element::new(
        WHITE,
        Vec::new(),
        Size::Percentage(glam::Vec2::new(50., 20.)),
    );
    let root_element = Element::new(
        RED,
        vec![child_element],
        Size::Percentage(glam::Vec2::new(100., 30.)),
    );

    let app = App::new(root_element);

    pollster::block_on(app.run());
}
