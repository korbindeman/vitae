use vitae::prelude::*;

const FILMSTRIP_HEIGHT: Length = Length::Px(200.0);
const THUMBNAIL_SIZE: Length = Length::Px(80.0);

#[derive(Clone)]
struct Model {
    images: Vec<String>,
    selected: usize,
}

fn view(model: &Model) -> ElementBuilder {
    div()
        .size(FULL)
        .bg(Color::from_hex("#1a1a1a"))
        .child(image_preview(model))
        .child(filmstrip_portal(model))
}

fn image_preview(model: &Model) -> ElementBuilder {
    let current = model.images.get(model.selected);

    div().size(FULL).child(
        div()
            .size(FULL)
            .bg(Color::from_hex("#2a2a2a"))
            .child(text(format!(
                "Image: {}",
                current.unwrap_or(&"(none)".to_string())
            ))),
    )
}

fn filmstrip_portal(model: &Model) -> ElementBuilder {
    portal()
        .left(px(0.0))
        .right(px(0.0))
        .bottom(px(0.0))
        .h(FILMSTRIP_HEIGHT)
        .child(filmstrip(model))
}

fn filmstrip(model: &Model) -> ElementBuilder {
    div()
        .size(FULL)
        .row()
        .bg(Color::from_hex("#333333"))
        .p(SM)
        .children(
            model
                .images
                .iter()
                .enumerate()
                .map(|(i, path)| thumbnail(i, path, i == model.selected)),
        )
}

fn thumbnail(index: usize, path: &str, selected: bool) -> ElementBuilder {
    let bg = if selected {
        Color::from_hex("#666666")
    } else {
        Color::from_hex("#444444")
    };

    div()
        .size(THUMBNAIL_SIZE)
        .bg(bg)
        .m(Length::Px(4.0))
        .child(text(format!("{}", index + 1)))
        .on_left_click(move |m: &mut Model| {
            m.selected = index;
        })
}

fn main() {
    let model = Model {
        images: vec![
            "photo1.jpg".to_string(),
            "photo2.jpg".to_string(),
            "photo3.jpg".to_string(),
            "photo4.jpg".to_string(),
            "photo5.jpg".to_string(),
        ],
        selected: 0,
    };

    App::new(model, view).run();
}
