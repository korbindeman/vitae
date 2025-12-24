use crate::builder::ElementBuilder;
use crate::texture::Texture;

/// Create an image element from a texture.
///
/// The element participates in layout like a normal div:
/// - No size specified: uses the texture's natural dimensions
/// - One dimension specified: preserves aspect ratio
/// - Both dimensions specified: stretches to fit
///
/// # Example
/// ```
/// let photo = load_texture("photo.png")?;
/// img(&photo).w(px(300.0))
/// ```
pub fn img(texture: &Texture) -> ElementBuilder {
    ElementBuilder::new_texture(texture.clone())
}
