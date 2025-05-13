use vitae::{App, immediate_ui::elements::Element};

fn main() {
    let child_element = Element::new("child", [1.0, 0.8, 0.2, 1.0], Vec::new(), 50, 20);
    let root_element = Element::new("root", [1.0, 0.6, 0.6, 1.0], vec![child_element], 100, 30);

    let app = App::new(root_element);

    pollster::block_on(app.run());
}
