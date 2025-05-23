mod utils;

use utils::checkerboard;
use vitae::{App, immediate_ui::elements::div::div};

fn main() {
    let root = div().col().children((0..8).map(|x| {
        div()
            .height(100. / 8.)
            .children((0..8).map(|y| div().background(checkerboard(x, y)).width(100. / 8.)))
    }));

    let app = App::new(root);

    app.run();
}
