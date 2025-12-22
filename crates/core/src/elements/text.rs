use crate::builder::ElementBuilder;

pub fn text(content: impl Into<String>) -> ElementBuilder {
    ElementBuilder::new_text(content.into())
}
