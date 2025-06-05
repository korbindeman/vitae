mod utils;

use utils::checkerboard;
use vitae::{
    App,
    immediate_ui::{color::Color, elements::div::div, style::px},
};

fn main() {
    let root = div().bg(Color::BLUE).col().children((0..8).map(|x| {
        div()
            .row()
            .h(px(1200. / 8.))
            .w(px(1200.))
            .children((0..8).map(|y| {
                div()
                    .bg(checkerboard(x, y))
                    .w(px(1200. / 8.))
                    .h(px(1200. / 8.))
            }))
    }));

    let app = App::new(root);

    app.run();
}
