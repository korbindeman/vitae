use vitae::prelude::*;

#[derive(Clone)]
struct Model {
    counter: i32,
    selected_tab: usize,
    items: Vec<String>,
    toggle_states: Vec<bool>,
}

impl Model {
    fn new() -> Self {
        Self {
            counter: 0,
            selected_tab: 0,
            items: vec![
                "Apple".to_string(),
                "Banana".to_string(),
                "Cherry".to_string(),
                "Date".to_string(),
                "Elderberry".to_string(),
            ],
            toggle_states: vec![false, true, false],
        }
    }
}

fn view(model: &Model) -> ElementBuilder {
    div()
        .size(FULL)
        .bg(Color::from_hex("#f5f5f5"))
        .col()
        .child(header())
        .child(
            div()
                .w(FULL)
                .h(FULL)
                .row()
                .child(sidebar(model))
                .child(main_content(model)),
        )
}

fn header() -> ElementBuilder {
    div()
        .w(FULL)
        .h(px(60.0))
        .bg(Color::from_hex("#2c3e50"))
        .row()
        .align(Align::Center)
        .distribute(Distribute::Between)
        .p(MD)
        .child(text("Vitae Kitchen Sink").font_size(24.0).bg(WHITE))
        .child(
            div()
                .row()
                .gap(SM)
                .child(nav_button("Home"))
                .child(nav_button("Docs"))
                .child(nav_button("About")),
        )
}

fn nav_button(label: &str) -> ElementBuilder {
    div()
        .bg(Color::from_hex("#34495e"))
        .p(SM)
        .child(text(label).bg(WHITE))
}

fn sidebar(model: &Model) -> ElementBuilder {
    let tabs = ["Layout", "Colors", "Alignment", "Interactive"];

    div()
        .w(px(200.0))
        .h(FULL)
        .bg(Color::from_hex("#ecf0f1"))
        .col()
        .p(SM)
        .gap(px(4.0))
        .children(tabs.iter().enumerate().map(|(i, label)| {
            let selected = i == model.selected_tab;
            div()
                .w(FULL)
                .bg(if selected {
                    Color::from_hex("#3498db")
                } else {
                    Color::from_hex("#bdc3c7")
                })
                .p(SM)
                .child(text(*label).bg(if selected { WHITE } else { BLACK }))
                .on_left_click(move |m: &mut Model| {
                    m.selected_tab = i;
                })
        }))
}

fn main_content(model: &Model) -> ElementBuilder {
    let content = match model.selected_tab {
        0 => layout_demo(),
        1 => colors_demo(),
        2 => alignment_demo(),
        3 => interactive_demo(model),
        _ => div(),
    };

    div().w(FULL).h(FULL).bg(WHITE).p(MD).child(content)
}

// ============================================================================
// Layout Demo
// ============================================================================

fn layout_demo() -> ElementBuilder {
    div()
        .size(FULL)
        .col()
        .gap(MD)
        .child(section_title("Layout Features"))
        // Row vs Column
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Row vs Column"))
                .child(
                    div()
                        .w(FULL)
                        .row()
                        .gap(MD)
                        .child(
                            div()
                                .w(px(200.0))
                                .h(px(100.0))
                                .bg(Color::from_hex("#e74c3c"))
                                .col()
                                .p(SM)
                                .gap(px(4.0))
                                .child(text("Column").bg(WHITE))
                                .child(colored_box("#c0392b", "A"))
                                .child(colored_box("#c0392b", "B")),
                        )
                        .child(
                            div()
                                .w(px(200.0))
                                .h(px(100.0))
                                .bg(Color::from_hex("#3498db"))
                                .row()
                                .p(SM)
                                .gap(px(4.0))
                                .child(text("Row").bg(WHITE))
                                .child(colored_box("#2980b9", "A"))
                                .child(colored_box("#2980b9", "B")),
                        ),
                ),
        )
        // Sizing
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Sizing: px(), pc(), FULL, HALF"))
                .child(
                    div()
                        .w(FULL)
                        .h(px(60.0))
                        .bg(Color::from_hex("#95a5a6"))
                        .row()
                        .gap(px(4.0))
                        .child(
                            div()
                                .w(px(100.0))
                                .h(FULL)
                                .bg(Color::from_hex("#9b59b6"))
                                .center()
                                .child(text("100px").bg(WHITE)),
                        )
                        .child(
                            div()
                                .w(pc(30.0))
                                .h(FULL)
                                .bg(Color::from_hex("#1abc9c"))
                                .center()
                                .child(text("30%").bg(WHITE)),
                        )
                        .child(
                            div()
                                .w(HALF)
                                .h(FULL)
                                .bg(Color::from_hex("#f39c12"))
                                .center()
                                .child(text("HALF").bg(WHITE)),
                        ),
                ),
        )
        // Padding and Margin
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Padding (p) and Margin (m)"))
                .child(
                    div()
                        .w(FULL)
                        .row()
                        .gap(MD)
                        .child(
                            div()
                                .bg(Color::from_hex("#e74c3c"))
                                .p(LG)
                                .child(text("p(LG)").bg(WHITE)),
                        )
                        .child(
                            div()
                                .bg(Color::from_hex("#3498db"))
                                .p(MD)
                                .child(text("p(MD)").bg(WHITE)),
                        )
                        .child(
                            div()
                                .bg(Color::from_hex("#2ecc71"))
                                .p(SM)
                                .child(text("p(SM)").bg(WHITE)),
                        )
                        .child(
                            div().bg(Color::from_hex("#9b59b6")).child(
                                div()
                                    .bg(Color::from_hex("#8e44ad"))
                                    .m(SM)
                                    .p(SM)
                                    .child(text("m(SM)").bg(WHITE)),
                            ),
                        ),
                ),
        )
        // Gap
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Gap between children"))
                .child(
                    div()
                        .w(FULL)
                        .row()
                        .gap(LG)
                        .child(
                            div()
                                .bg(Color::from_hex("#34495e"))
                                .row()
                                .gap(px(0.0))
                                .p(SM)
                                .child(small_box("#2c3e50"))
                                .child(small_box("#2c3e50"))
                                .child(small_box("#2c3e50")),
                        )
                        .child(
                            div()
                                .bg(Color::from_hex("#34495e"))
                                .row()
                                .gap(SM)
                                .p(SM)
                                .child(small_box("#2c3e50"))
                                .child(small_box("#2c3e50"))
                                .child(small_box("#2c3e50")),
                        )
                        .child(
                            div()
                                .bg(Color::from_hex("#34495e"))
                                .row()
                                .gap(MD)
                                .p(SM)
                                .child(small_box("#2c3e50"))
                                .child(small_box("#2c3e50"))
                                .child(small_box("#2c3e50")),
                        ),
                ),
        )
        // Aspect ratio
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Aspect Ratio: square(), aspect_ratio()"))
                .child(
                    div()
                        .row()
                        .gap(MD)
                        .align(Align::End)
                        .child(
                            div()
                                .h(px(80.0))
                                .square()
                                .bg(Color::from_hex("#e74c3c"))
                                .center()
                                .child(text("1:1").bg(WHITE)),
                        )
                        .child(
                            div()
                                .h(px(80.0))
                                .aspect_ratio(16.0 / 9.0)
                                .bg(Color::from_hex("#3498db"))
                                .center()
                                .child(text("16:9").bg(WHITE)),
                        )
                        .child(
                            div()
                                .h(px(80.0))
                                .aspect_ratio(4.0 / 3.0)
                                .bg(Color::from_hex("#2ecc71"))
                                .center()
                                .child(text("4:3").bg(WHITE)),
                        ),
                ),
        )
}

// ============================================================================
// Colors Demo
// ============================================================================

fn colors_demo() -> ElementBuilder {
    div()
        .size(FULL)
        .col()
        .gap(MD)
        .child(section_title("Color Features"))
        // Preset colors
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Preset Colors"))
                .child(
                    div()
                        .w(FULL)
                        .row()
                        .gap(SM)
                        .child(color_swatch(WHITE, "WHITE", true))
                        .child(color_swatch(BLACK, "BLACK", false))
                        .child(color_swatch(GRAY, "GRAY", false))
                        .child(color_swatch(RED, "RED", false))
                        .child(color_swatch(GREEN, "GREEN", false))
                        .child(color_swatch(BLUE, "BLUE", false))
                        .child(color_swatch(YELLOW, "YELLOW", true))
                        .child(color_swatch(CYAN, "CYAN", true))
                        .child(color_swatch(MAGENTA, "MAGENTA", false)),
                ),
        )
        // Hex colors
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Hex Colors (Color::from_hex)"))
                .child(
                    div()
                        .w(FULL)
                        .row()
                        .gap(SM)
                        .child(hex_swatch("#e74c3c", "Alizarin"))
                        .child(hex_swatch("#9b59b6", "Amethyst"))
                        .child(hex_swatch("#3498db", "Peter River"))
                        .child(hex_swatch("#1abc9c", "Turquoise"))
                        .child(hex_swatch("#2ecc71", "Emerald"))
                        .child(hex_swatch("#f1c40f", "Sun Flower"))
                        .child(hex_swatch("#e67e22", "Carrot"))
                        .child(hex_swatch("#34495e", "Wet Asphalt")),
                ),
        )
        // RGB colors
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("RGB Colors (Color::rgb)"))
                .child(
                    div()
                        .w(FULL)
                        .row()
                        .gap(SM)
                        .child(rgb_swatch(255, 0, 0))
                        .child(rgb_swatch(0, 255, 0))
                        .child(rgb_swatch(0, 0, 255))
                        .child(rgb_swatch(255, 128, 0))
                        .child(rgb_swatch(128, 0, 255))
                        .child(rgb_swatch(0, 255, 255)),
                ),
        )
        // Gradient simulation
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Color Gradient (simulated with boxes)"))
                .child(div().w(FULL).h(px(40.0)).row().children((0..20).map(|i| {
                    let t = i as f32 / 19.0;
                    let r = (255.0 * (1.0 - t)) as u8;
                    let b = (255.0 * t) as u8;
                    div().w(pc(5.0)).h(FULL).bg(Color::rgb(r, 0, b))
                }))),
        )
}

// ============================================================================
// Alignment Demo
// ============================================================================

fn alignment_demo() -> ElementBuilder {
    div()
        .size(FULL)
        .col()
        .gap(MD)
        .child(section_title("Alignment Features"))
        // Cross-axis alignment (align)
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Cross-axis Alignment (align)"))
                .child(
                    div()
                        .w(FULL)
                        .row()
                        .gap(MD)
                        .child(alignment_box("Start", Align::Start))
                        .child(alignment_box("Center", Align::Center))
                        .child(alignment_box("End", Align::End)),
                ),
        )
        // Main-axis distribution (distribute)
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Main-axis Distribution (distribute)"))
                .child(
                    div()
                        .w(FULL)
                        .col()
                        .gap(SM)
                        .child(distribute_box("Start", Distribute::Start))
                        .child(distribute_box("Center", Distribute::Center))
                        .child(distribute_box("End", Distribute::End))
                        .child(distribute_box("Between", Distribute::Between))
                        .child(distribute_box("Around", Distribute::Around))
                        .child(distribute_box("Evenly", Distribute::Evenly)),
                ),
        )
        // Center helper
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Center helper (centers both axes)"))
                .child(
                    div()
                        .w(px(300.0))
                        .h(px(100.0))
                        .bg(Color::from_hex("#34495e"))
                        .center()
                        .child(
                            div()
                                .bg(Color::from_hex("#e74c3c"))
                                .p(MD)
                                .child(text("Centered!").bg(WHITE)),
                        ),
                ),
        )
}

fn alignment_box(label: &str, align: Align) -> ElementBuilder {
    div()
        .w(px(120.0))
        .h(px(100.0))
        .bg(Color::from_hex("#34495e"))
        .col()
        .align(align)
        .p(SM)
        .child(text(label).bg(WHITE))
        .child(small_box("#e74c3c"))
}

fn distribute_box(label: &str, distribute: Distribute) -> ElementBuilder {
    div()
        .w(FULL)
        .h(px(50.0))
        .bg(Color::from_hex("#34495e"))
        .row()
        .distribute(distribute)
        .align(Align::Center)
        .p(SM)
        .child(
            div()
                .bg(Color::from_hex("#e74c3c"))
                .p(SM)
                .child(text(label).bg(WHITE)),
        )
        .child(small_box("#3498db"))
        .child(small_box("#2ecc71"))
}

// ============================================================================
// Interactive Demo
// ============================================================================

fn interactive_demo(model: &Model) -> ElementBuilder {
    let hover_state = use_signal(|| None::<usize>);

    div()
        .size(FULL)
        .col()
        .gap(MD)
        .child(section_title("Interactive Features"))
        // Counter
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Click Events (on_left_click)"))
                .child(
                    div()
                        .row()
                        .gap(SM)
                        .align(Align::Center)
                        .child(
                            div()
                                .bg(Color::from_hex("#e74c3c"))
                                .p(MD)
                                .child(text("-").bg(WHITE))
                                .on_left_click(|m: &mut Model| m.counter -= 1),
                        )
                        .child(
                            div()
                                .w(px(100.0))
                                .bg(Color::from_hex("#ecf0f1"))
                                .p(MD)
                                .center()
                                .child(text(format!("{}", model.counter))),
                        )
                        .child(
                            div()
                                .bg(Color::from_hex("#2ecc71"))
                                .p(MD)
                                .child(text("+").bg(WHITE))
                                .on_left_click(|m: &mut Model| m.counter += 1),
                        ),
                ),
        )
        // Toggles
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Toggle States"))
                .child(
                    div()
                        .row()
                        .gap(MD)
                        .children(model.toggle_states.iter().enumerate().map(|(i, &on)| {
                            div()
                                .w(px(80.0))
                                .h(px(40.0))
                                .bg(if on {
                                    Color::from_hex("#2ecc71")
                                } else {
                                    Color::from_hex("#e74c3c")
                                })
                                .center()
                                .child(text(if on { "ON" } else { "OFF" }).bg(WHITE))
                                .on_left_click(move |m: &mut Model| {
                                    m.toggle_states[i] = !m.toggle_states[i];
                                })
                        })),
                ),
        )
        // List with selection
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("List with Hover (use_signal)"))
                .child(
                    div()
                        .w(px(200.0))
                        .bg(Color::from_hex("#ecf0f1"))
                        .col()
                        .children(model.items.iter().enumerate().map(move |(i, item)| {
                            let is_hovered = hover_state.get() == Some(i);
                            div()
                                .w(FULL)
                                .bg(if is_hovered {
                                    Color::from_hex("#3498db")
                                } else {
                                    TRANSPARENT
                                })
                                .p(SM)
                                .child(text(item.clone()).bg(if is_hovered {
                                    WHITE
                                } else {
                                    BLACK
                                }))
                        })),
                ),
        )
        // Nested clickable
        .child(
            div()
                .w(FULL)
                .col()
                .gap(SM)
                .child(text("Nested Elements"))
                .child(
                    div()
                        .w(px(300.0))
                        .bg(Color::from_hex("#34495e"))
                        .p(MD)
                        .col()
                        .gap(SM)
                        .child(text("Outer container").bg(WHITE))
                        .child(
                            div()
                                .w(FULL)
                                .bg(Color::from_hex("#2c3e50"))
                                .p(SM)
                                .col()
                                .gap(SM)
                                .child(text("Inner container").bg(WHITE))
                                .child(
                                    div()
                                        .bg(Color::from_hex("#1a252f"))
                                        .p(SM)
                                        .child(text("Deepest").bg(WHITE)),
                                ),
                        ),
                ),
        )
}

// ============================================================================
// Helper Components
// ============================================================================

fn section_title(title: &str) -> ElementBuilder {
    div()
        .w(FULL)
        .p(SM)
        .bg(Color::from_hex("#2c3e50"))
        .child(text(title).font_size(20.0).bg(WHITE))
}

fn colored_box(hex: &str, label: &str) -> ElementBuilder {
    div()
        .size(px(30.0))
        .bg(Color::from_hex(hex))
        .center()
        .child(text(label).bg(WHITE))
}

fn small_box(hex: &str) -> ElementBuilder {
    div().size(px(30.0)).bg(Color::from_hex(hex))
}

fn color_swatch(color: Color, name: &str, dark_text: bool) -> ElementBuilder {
    div().w(px(70.0)).h(px(50.0)).bg(color).center().child(
        text(name)
            .font_size(10.0)
            .bg(if dark_text { BLACK } else { WHITE }),
    )
}

fn hex_swatch(hex: &str, name: &str) -> ElementBuilder {
    div()
        .w(px(80.0))
        .h(px(60.0))
        .bg(Color::from_hex(hex))
        .center()
        .col()
        .child(text(name).font_size(10.0).bg(WHITE))
        .child(text(hex).font_size(8.0).bg(WHITE))
}

fn rgb_swatch(r: u8, g: u8, b: u8) -> ElementBuilder {
    div()
        .w(px(90.0))
        .h(px(50.0))
        .bg(Color::rgb(r, g, b))
        .center()
        .child(
            text(format!("({},{},{})", r, g, b))
                .font_size(10.0)
                .bg(WHITE),
        )
}

fn main() {
    let app = App::new(Model::new(), view);
    app.run();
}
