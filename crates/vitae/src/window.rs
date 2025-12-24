use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key as WinitKey, NamedKey as WinitNamedKey};
use winit::window::{Window, WindowId};

use vitae_core::{ElementBuilder, Event, Key, MouseButton as VitaeMouseButton, NamedKey};
use vitae_render::Renderer;

use crate::signal::{reset_signal_counter, take_redraw_request};

fn convert_key(winit_key: &WinitKey) -> Key {
    match winit_key {
        WinitKey::Character(s) => Key::Character(s.to_string()),
        WinitKey::Named(named) => {
            let named_key = match named {
                WinitNamedKey::Enter => NamedKey::Enter,
                WinitNamedKey::Tab => NamedKey::Tab,
                WinitNamedKey::Space => NamedKey::Space,
                WinitNamedKey::Backspace => NamedKey::Backspace,
                WinitNamedKey::Delete => NamedKey::Delete,
                WinitNamedKey::Escape => NamedKey::Escape,
                WinitNamedKey::ArrowUp => NamedKey::ArrowUp,
                WinitNamedKey::ArrowDown => NamedKey::ArrowDown,
                WinitNamedKey::ArrowLeft => NamedKey::ArrowLeft,
                WinitNamedKey::ArrowRight => NamedKey::ArrowRight,
                WinitNamedKey::Home => NamedKey::Home,
                WinitNamedKey::End => NamedKey::End,
                WinitNamedKey::PageUp => NamedKey::PageUp,
                WinitNamedKey::PageDown => NamedKey::PageDown,
                WinitNamedKey::Shift => NamedKey::Shift,
                WinitNamedKey::Control => NamedKey::Control,
                WinitNamedKey::Alt => NamedKey::Alt,
                WinitNamedKey::Meta => NamedKey::Meta,
                WinitNamedKey::F1 => NamedKey::F1,
                WinitNamedKey::F2 => NamedKey::F2,
                WinitNamedKey::F3 => NamedKey::F3,
                WinitNamedKey::F4 => NamedKey::F4,
                WinitNamedKey::F5 => NamedKey::F5,
                WinitNamedKey::F6 => NamedKey::F6,
                WinitNamedKey::F7 => NamedKey::F7,
                WinitNamedKey::F8 => NamedKey::F8,
                WinitNamedKey::F9 => NamedKey::F9,
                WinitNamedKey::F10 => NamedKey::F10,
                WinitNamedKey::F11 => NamedKey::F11,
                WinitNamedKey::F12 => NamedKey::F12,
                _ => return Key::Unknown,
            };
            Key::Named(named_key)
        }
        _ => Key::Unknown,
    }
}

pub struct VitaeApp<'a, M: Clone> {
    renderer: Option<Renderer<'a>>,
    model: M,
    view_fn: fn(&M) -> ElementBuilder,
    cursor_position: (f64, f64),
    model_dirty: bool,
}

impl<'a, M: Clone + 'static> VitaeApp<'a, M> {
    pub fn new(initial_model: M, view: fn(&M) -> ElementBuilder) -> Self {
        Self {
            renderer: None,
            model: initial_model,
            view_fn: view,
            cursor_position: (0.0, 0.0),
            model_dirty: true,
        }
    }

    fn build_tree(&self) -> ElementBuilder {
        // Reset signal counter for consistent IDs across renders
        reset_signal_counter();
        (self.view_fn)(&self.model)
    }
}

impl<'a, M: Clone + 'static> ApplicationHandler for VitaeApp<'a, M> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("vitae"))
            .unwrap();
        let root = self.build_tree();
        self.renderer = Some(Renderer::new(window, root));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        if renderer.window().id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                renderer.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                // Only rebuild tree if model changed
                if self.model_dirty {
                    let root = self.build_tree();
                    if let Some(renderer) = self.renderer.as_mut() {
                        renderer.set_root(root);
                    }
                    self.model_dirty = false;
                }
                // Render (uses cached tree if clean)
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.render().unwrap();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = (position.x, position.y);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let vitae_button = match button {
                    MouseButton::Left => VitaeMouseButton::Left,
                    MouseButton::Right => VitaeMouseButton::Right,
                    MouseButton::Middle => VitaeMouseButton::Middle,
                    _ => return,
                };

                let (x, y) = self.cursor_position;
                let handler = renderer.hit_test(x as f32, y as f32);

                if let Some(handler) = handler {
                    let event = match state {
                        ElementState::Pressed => Event::MouseDown {
                            button: vitae_button,
                        },
                        ElementState::Released => Event::MouseUp {
                            button: vitae_button,
                        },
                    };
                    handler(&mut self.model, &event);

                    // Also fire Click on mouse up (left or right)
                    if matches!(state, ElementState::Released)
                        && matches!(
                            vitae_button,
                            VitaeMouseButton::Left | VitaeMouseButton::Right
                        )
                    {
                        handler(
                            &mut self.model,
                            &Event::Click {
                                button: vitae_button,
                            },
                        );
                    }

                    // Model was potentially modified
                    self.model_dirty = true;
                    if let Some(renderer) = self.renderer.as_ref() {
                        renderer.window().request_redraw();
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key = convert_key(&event.logical_key);
                let vitae_event = match event.state {
                    ElementState::Pressed => Event::KeyDown {
                        key,
                        repeat: event.repeat,
                    },
                    ElementState::Released => Event::KeyUp { key },
                };

                // For now, keyboard events go to the root element
                // TODO: implement focus system for targeted keyboard events
                let root_handler = renderer.get_root_handler();
                if let Some(handler) = root_handler {
                    handler(&mut self.model, &vitae_event);
                    // Model was potentially modified
                    self.model_dirty = true;
                    if let Some(renderer) = self.renderer.as_ref() {
                        renderer.window().request_redraw();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.as_ref() {
            // Check if any signal requested a redraw
            if take_redraw_request() {
                self.model_dirty = true;
                renderer.window().request_redraw();
            }
        }
    }
}
