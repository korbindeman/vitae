mod utils;

use utils::checkerboard;
use vitae::{
    App,
    core::{elements::div::div, style::pc},
};

fn main() {
    let root =
        div()
            .h(pc(100.))
            .aspect_ratio(1.0)
            .col()
            .children((0..8).map(|x| {
                div().row().h(pc(100. / 8.)).w(pc(100.)).children(
                    (0..8).map(|y| div().bg(checkerboard(x, y)).w(pc(100. / 8.)).h(pc(100.))),
                )
            }));

    let app = App::new(root);

    app.run();
}
