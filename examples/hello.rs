use vitae::prelude::*;

pub fn main() {
    // let root = div()
    //     .bg(Color::RED)
    //     .row()
    //     .w(pc(50.))
    //     .child(div().bg(Color::BLUE).w(pc(50.)).h(px(50.)))
    //     .child(div().bg(Color::CYAN).size(px(300.)));

    let root = div()
        .bg(Color::RED)
        .w(pc(100.))
        .aspect_ratio(16. / 9.)
        .child(div().bg(Color::BLUE).w(pc(50.)).h(px(50.)));

    let app = App::new(root);

    app.run();
}
