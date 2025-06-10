use vitae::prelude::*;

pub fn main() {
    let root = div().size(FULL).p(SM).child(text("Hello, Vitae!"));

    let app = App::new(root);

    app.run();
}
