mod utils;

use utils::checkerboard;
use vitae::prelude::*;

fn main() {
    let chessboard =
        div()
            .h(pc(100.))
            .aspect_ratio(1.0)
            .col()
            .children((0..8).map(|x| {
                div().row().h(pc(100. / 8.)).w(pc(100.)).children(
                    (0..8).map(|y| div().bg(checkerboard(x, y)).w(pc(100. / 8.)).h(pc(100.))),
                )
            }));

    let side_panel = div()
        .size(pc(100.))
        .bg(Color::GRAY)
        .child(div().bg(Color::BLUE).size(px(100.)));

    let root = div()
        .size(pc(100.))
        .row()
        .child(chessboard)
        .child(side_panel);

    let app = App::new(root);

    app.run();
}
