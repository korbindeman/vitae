# Images in Vitae

Vitae supports two types of images: raster images (PNG, JPEG, etc.) and vector graphics (SVG).

## Raster Images (Textures)

Use `load_texture()` to load raster images and `img()` to display them:

```rust
use vitae::prelude::*;

// Load a texture from a file
let photo = load_texture("assets/photo.png")?;

// Display it
img(&photo)
```

### Supported Formats

- PNG
- JPEG
- Other formats supported by the `image` crate

### Creating Textures from Raw Pixels

You can create textures dynamically from raw RGBA pixel data using `Texture::from_rgba()`:

```rust
use vitae::prelude::*;

// Create a 100x100 red square
let width = 100;
let height = 100;
let mut pixels = Vec::with_capacity((width * height * 4) as usize);

for _y in 0..height {
    for _x in 0..width {
        pixels.push(255); // R
        pixels.push(0);   // G
        pixels.push(0);   // B
        pixels.push(255); // A
    }
}

let texture = Texture::from_rgba(pixels, width, height);
img(&texture)
```

This is useful for:

- Procedurally generated graphics
- Rendering from other sources (e.g., video frames, canvas drawing)
- Dynamic visualizations

The pixel data must be:
- RGBA format (4 bytes per pixel)
- Row-major order (left-to-right, top-to-bottom)
- Exactly `width * height * 4` bytes

### Sizing Behavior

Textures participate in layout like normal elements:

```rust
// Natural size (uses image dimensions)
img(&photo)

// Fixed width, height scales to preserve aspect ratio
img(&photo).w(px(300.0))

// Fixed height, width scales to preserve aspect ratio
img(&photo).h(px(200.0))

// Both dimensions specified, stretches to fit
img(&photo).w(px(300.0)).h(px(200.0))

// Percentage-based sizing
img(&photo).w(pc(50.0))
```

## Vector Graphics (SVG)

Use `load_svg()` to load SVG files and `svg()` to display them:

```rust
use vitae::prelude::*;

// Load an SVG from a file
let icon = load_svg("assets/icon.svg")?;

// Display it
svg(&icon)
```

### Sizing Behavior

SVGs behave the same as textures for sizing:

```rust
// Natural size (uses SVG viewBox dimensions)
svg(&icon)

// Fixed width, height scales to preserve aspect ratio
svg(&icon).w(px(64.0))

// Fixed height, width scales to preserve aspect ratio
svg(&icon).h(px(64.0))

// Both dimensions specified, scales to fit
svg(&icon).w(px(100.0)).h(px(100.0))

// Percentage-based sizing
svg(&icon).size(pc(80.0))
```

### Why SVG?

SVGs are vector graphics that scale perfectly at any size without pixelation. Use them for:

- Icons
- Logos
- UI elements that need to scale
- Any graphics with clean lines and solid colors

## Example: Loading Multiple Images

```rust
use std::collections::HashMap;

#[derive(Clone)]
struct App {
    icons: HashMap<String, Svg>,
}

impl App {
    fn new() -> Self {
        let mut icons = HashMap::new();
        for name in ["home", "settings", "user"] {
            if let Ok(svg) = load_svg(&format!("assets/icons/{}.svg", name)) {
                icons.insert(name.to_string(), svg);
            }
        }
        Self { icons }
    }
}

fn view(app: &App) -> ElementBuilder {
    div().row().children(
        app.icons.iter().map(|(name, icon)| {
            svg(icon).w(px(24.0))
        })
    )
}
```

## Performance Notes

- Textures are decoded to RGBA pixels when loaded
- SVGs are parsed and rendered each frame (consider caching for complex SVGs)
- Both types use GPU acceleration for rendering
