use vitae::prelude::*;

pub fn main() {
    let root = div()
        .size(FULL)
        .bg(RED)
        .child(div().size(px(400.)).child(text("Hello from Vitae")));

    let app = App::new(root);

    app.run();
}
