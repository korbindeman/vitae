pub mod immediate_ui;
mod renderer_wgpu;
mod window;

use immediate_ui::builder::ElementBuilder;
use window::StateApplication;
use winit::event_loop::EventLoop;

pub struct App {
    event_loop: EventLoop<()>,
    window_state: StateApplication<'static>,
}

impl App {
    pub fn new(root_element: ElementBuilder) -> Self {
        App {
            event_loop: EventLoop::new().unwrap(),
            window_state: StateApplication::new(root_element),
        }
    }

    pub fn run(self) {
        pollster::block_on(self.run_event_loop());
    }

    async fn run_event_loop(mut self) {
        let _ = self.event_loop.run_app(&mut self.window_state);
    }
}
