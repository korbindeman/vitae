use vitae::{
    App,
    immediate_ui::{
        color::Color,
        elements::div::div,
        style::{pc, px},
    },
};

pub fn main() {
    let root = div()
        .bg(Color::RED)
        .row()
        .w(pc(50.))
        .child(div().bg(Color::BLUE).w(pc(50.)).h(px(50.)))
        .child(div().bg(Color::CYAN).size(px(300.)));

    let app = App::new(root);

    app.run();
}
