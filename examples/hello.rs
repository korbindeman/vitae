use tracing::warn;
use vitae::{
    App,
    immediate_ui::{color::Color, elements::div::div, style::px},
};

pub fn main() {
    let root = div()
        .col()
        .p(px(20.))
        .w(px(800.))
        .h(px(1200.))
        .child(div().w(px(400.)).h(px(100.)).bg(Color::BLUE))
        .child(div().w(px(300.)).h(px(100.)).bg(Color::GREEN))
        .child(div().w(px(200.)).h(px(100.)).bg(Color::RED))
        .child(div().w(px(100.)).h(px(100.)).bg(Color::MAGENTA))
        .child(div().h(px(1000.)).w(px(10.)).bg(Color::CYAN));

    let app = App::new(root);

    app.run();
}
