use std::path::Path;

use vitae_core::{Svg, Texture};

/// Load a texture from an image file.
///
/// Supports PNG, JPEG, and other common image formats.
/// The image is decoded to RGBA pixels.
///
/// # Example
/// ```no_run
/// let texture = load_texture("photo.png")?;
/// ```
pub fn load_texture<P: AsRef<Path>>(path: P) -> Result<Texture, image::ImageError> {
    let img = image::open(path)?;
    let rgba = img.into_rgba8();
    let (width, height) = rgba.dimensions();
    Ok(Texture::from_rgba(rgba.into_raw(), width, height))
}

/// Load an SVG from a file.
///
/// # Example
/// ```no_run
/// let icon = load_svg("icon.svg")?;
/// ```
pub fn load_svg<P: AsRef<Path>>(path: P) -> Result<Svg, std::io::Error> {
    let data = std::fs::read_to_string(path)?;
    let tree = vello_svg::usvg::Tree::from_str(&data, &vello_svg::usvg::Options::default())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let size = tree.size();
    Ok(Svg::new(data, size.width(), size.height()))
}
