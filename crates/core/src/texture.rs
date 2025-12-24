/// A texture holding RGBA pixel data.
///
/// Textures can be displayed using the `img()` element helper.
/// They participate in layout like normal elements - if no size is specified,
/// they use their natural dimensions; if one dimension is specified, aspect
/// ratio is preserved; if both are specified, the texture stretches to fit.
#[derive(Clone, Debug)]
pub struct Texture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Texture {
    /// Create a texture from raw RGBA pixel data.
    ///
    /// # Arguments
    /// * `data` - RGBA pixels, 4 bytes per pixel, row-major order
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    ///
    /// # Panics
    /// Panics if `data.len() != width * height * 4`
    pub fn from_rgba(data: Vec<u8>, width: u32, height: u32) -> Self {
        assert_eq!(
            data.len(),
            (width * height * 4) as usize,
            "Texture data size mismatch: expected {} bytes for {}x{} RGBA, got {}",
            width * height * 4,
            width,
            height,
            data.len()
        );
        Self {
            data,
            width,
            height,
        }
    }

    /// Get the width of the texture in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the texture in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get the aspect ratio (width / height).
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    /// Get the raw RGBA pixel data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
