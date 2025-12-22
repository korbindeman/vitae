pub mod prelude;
pub mod signal;
mod window;

pub use vitae_core as core;
pub use vitae_render as render;

use vitae_core::ElementBuilder;
use window::VitaeApp;
use winit::event_loop::EventLoop;

pub use signal::{use_signal, Signal};

pub struct App<M: Clone + 'static> {
    event_loop: EventLoop<()>,
    vitae_app: VitaeApp<'static, M>,
}

impl<M: Clone + 'static> App<M> {
    /// Create a new application with a model and view function
    ///
    /// # Arguments
    /// * `initial_model` - The initial state of your application
    /// * `view` - A function that takes a reference to the model and returns the UI tree
    ///
    /// # Example
    /// ```
    /// #[derive(Clone)]
    /// struct Counter { count: i32 }
    ///
    /// fn view(model: &Counter) -> ElementBuilder {
    ///     div().child(text(format!("Count: {}", model.count)))
    /// }
    ///
    /// App::new(Counter { count: 0 }, view).run();
    /// ```
    pub fn new(initial_model: M, view: fn(&M) -> ElementBuilder) -> Self {
        App {
            event_loop: EventLoop::new().unwrap(),
            vitae_app: VitaeApp::new(initial_model, view),
        }
    }

    pub fn run(mut self) {
        let _ = self.event_loop.run_app(&mut self.vitae_app);
    }
}
