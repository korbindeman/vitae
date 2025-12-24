use std::borrow::Cow;
use std::sync::Arc;

use parley::{FontContext, LayoutContext, LineHeight, StyleProperty};
use pollster::FutureExt;
use vello::kurbo::{Affine, Rect};
use vello::peniko::{color::palette, Fill};
use vello::wgpu::{self, CommandEncoderDescriptor};
use vello::{AaConfig, NormalizedCoord, RenderParams, RendererOptions, Scene};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use vitae_core::{
    layout, Constraints, ElementBuilder, ElementTree, NodeId, NodeKind, Position, TextMeasurer,
};

// Sensible defaults (TODO: replace with theme system)
const DEFAULT_FONT_SIZE: f32 = 24.0;

/// Text measurer that uses Parley for font-aware text measurement.
struct ParleyMeasurer<'a> {
    font_cx: &'a mut FontContext,
    layout_cx: &'a mut LayoutContext<()>,
    font_size: f32,
}

impl TextMeasurer for ParleyMeasurer<'_> {
    fn measure(&mut self, text: &str, max_width: Option<f32>) -> (f32, f32) {
        let mut builder = self.layout_cx.ranged_builder(self.font_cx, text, 1.0, true);

        // Use font stack with system UI font first, then symbol fonts as fallback
        // This way regular text uses the nice system font, but chess symbols still work
        builder.push_default(StyleProperty::FontStack(parley::style::FontStack::List(
            Cow::Borrowed(&[
                parley::style::FontFamily::Generic(parley::style::GenericFamily::SystemUi),
                parley::style::FontFamily::Named(Cow::Borrowed("Noto Sans Symbols 2")),
                parley::style::FontFamily::Named(Cow::Borrowed("Segoe UI Symbol")),
                parley::style::FontFamily::Named(Cow::Borrowed("Apple Symbols")),
                parley::style::FontFamily::Generic(parley::style::GenericFamily::SansSerif),
            ]),
        )));

        builder.push_default(StyleProperty::FontSize(self.font_size));
        let mut text_layout = builder.build(text);
        text_layout.break_all_lines(max_width);

        (text_layout.width(), text_layout.height())
    }
}

pub struct Renderer<'a> {
    // Vello rendering
    context: vello::util::RenderContext,
    surface: vello::util::RenderSurface<'a>,
    vello_renderer: vello::Renderer,
    scene: Scene,

    // Text
    font_cx: FontContext,
    layout_cx: LayoutContext<()>,

    // Window state
    size: PhysicalSize<u32>,
    window: Arc<Window>,

    // UI tree
    root_element: ElementBuilder,
}

impl<'a> Renderer<'a> {
    pub fn new(window: Window, root_element: ElementBuilder) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();

        let mut context = vello::util::RenderContext::new();

        let surface = context
            .create_surface(
                window.clone(),
                size.width,
                size.height,
                wgpu::PresentMode::AutoVsync,
            )
            .block_on()
            .expect("Failed to create surface");

        let device = &context.devices[surface.dev_id].device;

        let vello_renderer = vello::Renderer::new(device, RendererOptions::default())
            .expect("Failed to create Vello renderer");

        let font_cx = FontContext::new();
        let layout_cx = LayoutContext::new();

        Self {
            context,
            surface,
            vello_renderer,
            scene: Scene::new(),
            font_cx,
            layout_cx,
            size,
            window,
            root_element,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.context
                .resize_surface(&mut self.surface, new_size.width, new_size.height);
        }
    }

    /// Update the root element (used when model/signals change)
    pub fn set_root(&mut self, root_element: ElementBuilder) {
        self.root_element = root_element;
    }

    pub fn render(&mut self) -> Result<(), vello::wgpu::SurfaceError> {
        // Build and layout the element tree
        let mut tree = self.root_element.clone().build();
        let root = tree.root;

        // Create text measurer for layout
        let mut measurer = ParleyMeasurer {
            font_cx: &mut self.font_cx,
            layout_cx: &mut self.layout_cx,
            font_size: DEFAULT_FONT_SIZE,
        };

        layout(
            &mut tree,
            root,
            Constraints {
                max_w: self.size.width as f32,
                max_h: self.size.height as f32,
            },
            0.0,
            0.0,
            &mut measurer,
        );

        // Build the Vello scene from the tree
        self.scene.reset();
        let mut portals = Vec::new();
        self.render_node(&tree, root, &mut portals);

        // Render portals last (on top of everything)
        for portal_id in portals {
            self.render_node_and_children(&tree, portal_id);
        }

        // Render to surface
        let device_handle = &self.context.devices[self.surface.dev_id];

        self.vello_renderer
            .render_to_texture(
                &device_handle.device,
                &device_handle.queue,
                &self.scene,
                &self.surface.target_view,
                &RenderParams {
                    base_color: palette::css::WHITE,
                    width: self.size.width,
                    height: self.size.height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .expect("Failed to render to surface");

        let surface_texture = self.surface.surface.get_current_texture()?;
        let mut encoder = device_handle
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Blit encoder"),
            });
        self.surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &self.surface.target_view,
            &surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );
        device_handle.queue.submit(Some(encoder.finish()));
        surface_texture.present();

        Ok(())
    }

    fn render_node(&mut self, tree: &ElementTree, id: NodeId, portals: &mut Vec<NodeId>) {
        let node = tree.get_node(id);
        let layout = node.layout;

        match &node.kind {
            NodeKind::Element { style } => {
                let color = style.bg_color.to_array();
                // Skip fully transparent elements
                if color[3] > 0.0 {
                    let rect = Rect::new(
                        layout.x as f64,
                        layout.y as f64,
                        (layout.x + layout.width) as f64,
                        (layout.y + layout.height) as f64,
                    );
                    let vello_color =
                        vello::peniko::Color::new([color[0], color[1], color[2], color[3]]);
                    self.scene
                        .fill(Fill::NonZero, Affine::IDENTITY, vello_color, None, &rect);
                }
            }
            NodeKind::Text { content, style } => {
                let text_color = style.text_color.to_array();
                let font_size = style.font_size.unwrap_or(DEFAULT_FONT_SIZE);
                self.render_text(
                    content,
                    layout.x,
                    layout.y,
                    layout.width,
                    font_size,
                    [text_color[0], text_color[1], text_color[2], text_color[3]],
                );
            }
        }

        // Render children, collecting portals
        let mut child = node.first_child;
        while let Some(child_id) = child {
            let child_node = tree.get_node(child_id);
            if let Some(style) = child_node.style() {
                if style.position == Position::Portal {
                    portals.push(child_id);
                    child = child_node.next_sibling;
                    continue;
                }
            }
            self.render_node(tree, child_id, portals);
            child = tree.get_node(child_id).next_sibling;
        }
    }

    /// Render a node and all its children (used for portals, no portal collection).
    fn render_node_and_children(&mut self, tree: &ElementTree, id: NodeId) {
        let node = tree.get_node(id);
        let layout = node.layout;

        match &node.kind {
            NodeKind::Element { style } => {
                let color = style.bg_color.to_array();
                if color[3] > 0.0 {
                    let rect = Rect::new(
                        layout.x as f64,
                        layout.y as f64,
                        (layout.x + layout.width) as f64,
                        (layout.y + layout.height) as f64,
                    );
                    let vello_color =
                        vello::peniko::Color::new([color[0], color[1], color[2], color[3]]);
                    self.scene
                        .fill(Fill::NonZero, Affine::IDENTITY, vello_color, None, &rect);
                }
            }
            NodeKind::Text { content, style } => {
                let text_color = style.text_color.to_array();
                let font_size = style.font_size.unwrap_or(DEFAULT_FONT_SIZE);
                self.render_text(
                    content,
                    layout.x,
                    layout.y,
                    layout.width,
                    font_size,
                    [text_color[0], text_color[1], text_color[2], text_color[3]],
                );
            }
        }

        let mut child = node.first_child;
        while let Some(child_id) = child {
            self.render_node_and_children(tree, child_id);
            child = tree.get_node(child_id).next_sibling;
        }
    }

    fn render_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        max_width: f32,
        font_size: f32,
        color: [f32; 4],
    ) {
        let line_height = 1.2;

        let mut builder = self
            .layout_cx
            .ranged_builder(&mut self.font_cx, text, 1.0, true);

        // Set font family stack with system UI font first, then symbol fonts as fallback
        // This way regular text uses the nice system font, but chess symbols still work
        builder.push_default(StyleProperty::FontStack(parley::style::FontStack::List(
            Cow::Borrowed(&[
                parley::style::FontFamily::Generic(parley::style::GenericFamily::SystemUi),
                parley::style::FontFamily::Named(Cow::Borrowed("Noto Sans Symbols 2")),
                parley::style::FontFamily::Named(Cow::Borrowed("Segoe UI Symbol")),
                parley::style::FontFamily::Named(Cow::Borrowed("Apple Symbols")),
                parley::style::FontFamily::Generic(parley::style::GenericFamily::SansSerif),
            ]),
        )));

        builder.push_default(StyleProperty::FontSize(font_size));
        builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(
            line_height,
        )));
        let mut text_layout = builder.build(text);
        text_layout.break_all_lines(Some(max_width));

        let text_color = vello::peniko::Color::new(color);

        for line in text_layout.lines() {
            for item in line.items() {
                if let parley::PositionedLayoutItem::GlyphRun(glyph_run) = item {
                    let run = glyph_run.run();
                    let font = run.font();
                    let font_size = run.font_size();
                    let synthesis = run.synthesis();
                    let glyph_xform = synthesis
                        .skew()
                        .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));
                    let coords: Vec<NormalizedCoord> =
                        run.normalized_coords().iter().copied().collect();

                    // Starting position for this glyph run
                    let mut gx = x + glyph_run.offset();
                    let gy = y + glyph_run.baseline();

                    self.scene
                        .draw_glyphs(font)
                        .font_size(font_size)
                        .transform(Affine::IDENTITY)
                        .glyph_transform(glyph_xform)
                        .normalized_coords(&coords)
                        .brush(text_color)
                        .draw(
                            Fill::NonZero,
                            glyph_run.glyphs().map(|g| {
                                let pos_x = gx + g.x;
                                let pos_y = gy - g.y;
                                gx += g.advance;
                                vello::Glyph {
                                    id: g.id,
                                    x: pos_x,
                                    y: pos_y,
                                }
                            }),
                        );
                }
            }
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Perform hit-testing to find which element was clicked
    /// Returns the event handler if an element with a click handler was hit
    pub fn hit_test(&mut self, x: f32, y: f32) -> Option<vitae_core::EventHandler> {
        // Build and layout the tree to get correct positions
        let mut tree = self.root_element.clone().build();
        let root = tree.root;

        // Run layout to calculate positions
        let mut measurer = ParleyMeasurer {
            font_cx: &mut self.font_cx,
            layout_cx: &mut self.layout_cx,
            font_size: DEFAULT_FONT_SIZE,
        };

        layout(
            &mut tree,
            root,
            Constraints {
                max_w: self.size.width as f32,
                max_h: self.size.height as f32,
            },
            0.0,
            0.0,
            &mut measurer,
        );

        // Collect portals first, then check them (they're rendered on top)
        let mut portals = Vec::new();
        self.collect_portals(&tree, tree.root, &mut portals);

        // Check portals first (last rendered = frontmost)
        for portal_id in portals.iter().rev() {
            if let Some(handler) = self.hit_test_node_all(&tree, *portal_id, x, y) {
                return Some(handler);
            }
        }

        // Then check the normal tree
        self.hit_test_node(&tree, tree.root, x, y, &portals)
    }

    fn collect_portals(
        &self,
        tree: &vitae_core::ElementTree,
        node_id: vitae_core::NodeId,
        portals: &mut Vec<vitae_core::NodeId>,
    ) {
        let node = tree.get_node(node_id);

        let mut child = node.first_child;
        while let Some(child_id) = child {
            let child_node = tree.get_node(child_id);
            if let Some(style) = child_node.style() {
                if style.position == Position::Portal {
                    portals.push(child_id);
                    child = child_node.next_sibling;
                    continue;
                }
            }
            self.collect_portals(tree, child_id, portals);
            child = tree.get_node(child_id).next_sibling;
        }
    }

    fn hit_test_node(
        &self,
        tree: &vitae_core::ElementTree,
        node_id: vitae_core::NodeId,
        x: f32,
        y: f32,
        portals: &[vitae_core::NodeId],
    ) -> Option<vitae_core::EventHandler> {
        let node = tree.get_node(node_id);
        let layout = &node.layout;

        // Check if point is inside this node's bounds
        let in_bounds = x >= layout.x
            && x <= layout.x + layout.width
            && y >= layout.y
            && y <= layout.y + layout.height;

        if !in_bounds {
            return None;
        }

        // Check children first (they're on top), skipping portals
        let mut child = node.first_child;
        while let Some(child_id) = child {
            // Skip portals - they're handled separately
            if portals.contains(&child_id) {
                child = tree.get_node(child_id).next_sibling;
                continue;
            }
            if let Some(handler) = self.hit_test_node(tree, child_id, x, y, portals) {
                return Some(handler);
            }
            child = tree.get_node(child_id).next_sibling;
        }

        // If no child was hit, check if this node has a handler
        node.on_event.clone()
    }

    /// Hit test a node and all children (used for portals, no skipping)
    fn hit_test_node_all(
        &self,
        tree: &vitae_core::ElementTree,
        node_id: vitae_core::NodeId,
        x: f32,
        y: f32,
    ) -> Option<vitae_core::EventHandler> {
        let node = tree.get_node(node_id);
        let layout = &node.layout;

        let in_bounds = x >= layout.x
            && x <= layout.x + layout.width
            && y >= layout.y
            && y <= layout.y + layout.height;

        if !in_bounds {
            return None;
        }

        let mut child = node.first_child;
        while let Some(child_id) = child {
            if let Some(handler) = self.hit_test_node_all(tree, child_id, x, y) {
                return Some(handler);
            }
            child = tree.get_node(child_id).next_sibling;
        }

        node.on_event.clone()
    }

    /// Get the event handler for the root element.
    pub fn get_root_handler(&self) -> Option<vitae_core::EventHandler> {
        self.root_element.get_event_handler()
    }
}
