use vitae::prelude::*;

fn checkerboard(x: i32, y: i32) -> Color {
    let light_square = Color::rgb(242, 229, 229);
    let dark_square = Color::rgb(163, 82, 76);

    if ((x + y) & 1) == 0 {
        light_square
    } else {
        dark_square
    }
}

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
        .bg(WHITE)
        .p(px(12.0))
        .col()
        .child(text("Chess"))
        .child(text("White to move"))
        .child(text("Last move: e2-e4"));

    let root = div().size(FULL).row().child(chessboard).child(side_panel);

    let app = App::new(root);

    app.run();
}
