mod utils;

use utils::checkerboard;
use vitae::prelude::*;

fn main() {
    let chessboard = div().h(FULL).square().col().children((0..8).map(|x| {
        div()
            .row()
            .h(pc(100. / 8.))
            .w(FULL)
            .children((0..8).map(|y| div().bg(checkerboard(x, y)).w(pc(100. / 8.)).h(FULL)))
    }));

    let side_panel = div()
        .size(FULL)
        .bg(GRAY)
        .p(px(12.0))
        .child(div().bg(BLUE).size(px(100.)))
        .child(text("Hello world"));

    let root = div().size(FULL).row().child(chessboard).child(side_panel);

    let app = App::new(root);

    app.run();
}
