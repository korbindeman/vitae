use crate::builder::ElementBuilder;
use crate::style::Position;

/// Create a portal element that is positioned relative to the viewport
/// and rendered on top of all other content.
pub fn portal() -> ElementBuilder {
    ElementBuilder::new().position(Position::Portal)
}
