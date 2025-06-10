use std::sync::Arc;

use glyphon::{
    Attrs, Buffer, Cache, Color, FontSystem, Metrics, Shaping, SwashCache, TextArea, TextAtlas,
    TextBounds, TextRenderer, Viewport,
};
use pollster::FutureExt;
use wgpu::util::DeviceExt;
use wgpu::{Adapter, Device, Instance, PresentMode, Queue, Surface, SurfaceCapabilities};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::core::builder::ElementBuilder;
use crate::core::draw::push_draw_commands;
use crate::core::layout::{Constraints, layout};

use super::vertex::{Vertex, build_mesh};

pub struct State<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,

    size: PhysicalSize<u32>,
    scale_factor: f64,
    window: Arc<Window>,

    root_element: ElementBuilder,

    render_pipeline: wgpu::RenderPipeline,

    // buffers for ui mesh
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    // glyphon fields for text rendering
    font_system: FontSystem,
    swash_cache: SwashCache,
    cache: Cache,
    text_atlas: TextAtlas,
    text_renderer: TextRenderer,
    // a buffer to hold the text we want to render
    text_buffer: Buffer,
    viewport: Viewport,
}

impl<'a> State<'a> {
    pub fn new(window: Window, root_element: ElementBuilder) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        let instance = Self::create_gpu_instance();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = Self::create_adapter(instance, &surface);
        let (device, queue) = Self::create_device(&adapter);
        let surface_caps = surface.get_capabilities(&adapter);
        let config = Self::create_surface_config(size, surface_caps);
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Render Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // use alpha blending for text
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // --- Glyphon Text Rendering Setup ---
        // Create a FontSystem, which serves as a database of fonts.
        // You need to load your fonts into this system.
        let mut font_system = FontSystem::new();
        // TODO: Load your font data. Here's an example of how you might do it.
        // You need to have a font file (e.g., a .ttf or .otf) available.
        // let font_data = include_bytes!("../path/to/your/font.ttf");
        // font_system.db_mut().load_font_data(font_data.to_vec());

        // a SwashCache is used for glyph rasterization.
        let swash_cache = SwashCache::new();

        // a Cache is used for storing glyph cache data.
        let cache = Cache::new(&device);

        // a TextAtlas is the GPU texture that holds all of the rendered glyphs.
        let mut text_atlas = TextAtlas::new(&device, &queue, &cache, config.format);

        // the TextRenderer is responsible for drawing the text from the atlas.
        let text_renderer = TextRenderer::new(
            &mut text_atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );

        // create a viewport for text rendering
        let viewport = Viewport::new(&device, &cache);

        // this buffer will be populated in `rebuild_layout_and_assets`
        let text_buffer = Buffer::new(&mut font_system, Metrics::new(24.0, 32.0));

        // create dummy buffers before moving device into the struct
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 0,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 0,
            usage: wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
        });

        let mut s = Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            scale_factor,
            render_pipeline,
            root_element,
            vertex_buffer,
            index_buffer,
            num_indices: 0,
            font_system,
            swash_cache,
            cache,
            text_atlas,
            text_renderer,
            text_buffer,
            viewport,
        };

        // Perform initial layout and create all GPU assets
        s.rebuild_layout_and_assets();

        s
    }

    fn create_surface_config(
        size: PhysicalSize<u32>,
        capabilities: SurfaceCapabilities,
    ) -> wgpu::SurfaceConfiguration {
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn create_device(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .block_on()
            .unwrap()
    }

    fn create_adapter(instance: Instance, surface: &Surface) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap()
    }

    fn create_gpu_instance() -> Instance {
        Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.rebuild_layout_and_assets();
            println!("Resized to {:?}", new_size);
        }
    }

    pub fn scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // --- Prepare Text ---
        // This step takes the text buffers, determines which glyphs are needed,
        // and uploads them to the GPU texture atlas. This should be done
        // BEFORE the render pass.

        // Update viewport first
        self.viewport.update(
            &self.queue,
            glyphon::Resolution {
                width: self.config.width,
                height: self.config.height,
            },
        );

        self.text_renderer
            .prepare(
                &self.device,
                &self.queue,
                &mut self.font_system,
                &mut self.text_atlas,
                &self.viewport,
                // Define the areas of text to draw. We are just drawing our one buffer.
                [TextArea {
                    buffer: &self.text_buffer,
                    left: 10.0, // X position
                    top: 10.0,  // Y position
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: self.size.width as i32,
                        bottom: self.size.height as i32,
                    },
                    default_color: Color::rgb(255, 255, 255),
                    custom_glyphs: &[],
                }],
                &mut self.swash_cache,
            )
            .unwrap();

        // --- Render Frame ---
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI & Text Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            // draw your existing UI elements
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

            // draw the text
            self.text_renderer
                .render(&self.text_atlas, &self.viewport, &mut render_pass)
                .unwrap();
        }

        self.queue.submit(Some(encoder.finish()));
        self.text_atlas.trim();
        frame.present();

        Ok(())
    }

    /// Re-runs the layout engine and rebuilds all GPU buffers for UI and text.
    fn rebuild_layout_and_assets(&mut self) {
        // --- 1. Rebuild UI Mesh ---
        let mut tree = self.root_element.clone().build();
        let root = tree.root;
        layout(
            &mut tree,
            root,
            Constraints {
                max_h: self.size.height as f32,
                max_w: self.size.width as f32,
            },
            0.0,
            0.0,
        );
        let mut cmds = Vec::new();
        push_draw_commands(
            &tree,
            tree.root,
            &mut cmds,
            self.size.width as f32,
            self.size.height as f32,
        );
        let (verts, inds) = build_mesh(cmds.as_slice());

        self.vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });

        self.index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&inds),
                usage: wgpu::BufferUsages::INDEX,
            });

        self.num_indices = inds.len() as u32;

        // --- 2. Rebuild Text Buffer ---
        // This is where you would update your text content based on application state.

        // Set the buffer's size to the window size. This is important for text wrapping.
        self.text_buffer.set_size(
            &mut self.font_system,
            Some(self.size.width as f32),
            Some(self.size.height as f32),
        );

        // Clear previous text and set new text.
        self.text_buffer.lines.clear();
        self.text_buffer.set_text(
            &mut self.font_system,
            "Hello, wgpu! This is a test of the glyphon text rendering library.\nNew lines and wrapping should work correctly.",
            &Attrs::new().color(Color::rgb(255, 255, 255)),
            Shaping::Advanced,
        );
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
