/// An SVG image that can be rendered at any size.
///
/// SVGs can be displayed using the `svg()` element helper.
/// They participate in layout like normal elements - if no size is specified,
/// they use their natural dimensions; if one dimension is specified, aspect
/// ratio is preserved; if both are specified, the SVG scales to fit.
#[derive(Clone, Debug)]
pub struct Svg {
    data: String,
    width: f32,
    height: f32,
}

impl Svg {
    /// Create an SVG from raw SVG data.
    ///
    /// # Arguments
    /// * `data` - The SVG content as a string
    /// * `width` - Natural width from the SVG viewBox
    /// * `height` - Natural height from the SVG viewBox
    pub fn new(data: String, width: f32, height: f32) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    /// Get the natural width of the SVG.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get the natural height of the SVG.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Get the aspect ratio (width / height).
    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height
    }

    /// Get the raw SVG data.
    pub fn data(&self) -> &str {
        &self.data
    }
}
