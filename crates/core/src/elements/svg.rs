use crate::builder::ElementBuilder;
use crate::svg_data::Svg;

/// Create an SVG image element.
///
/// The element participates in layout like a normal div:
/// - No size specified: uses the SVG's natural dimensions
/// - One dimension specified: preserves aspect ratio
/// - Both dimensions specified: scales to fit
///
/// # Example
/// ```
/// let icon = load_svg("icon.svg")?;
/// svg_img(&icon).w(px(64.0))
/// ```
pub fn svg(svg_data: &Svg) -> ElementBuilder {
    ElementBuilder::new_svg(svg_data.clone())
}
