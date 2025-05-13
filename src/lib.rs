pub mod immediate_ui;
mod renderer_wgpu;
mod window;

use immediate_ui::elements::Element;
use window::StateApplication;
use winit::event_loop::EventLoop;

pub struct App {
    root_element: Element,
    event_loop: EventLoop<()>,
    window_state: StateApplication<'static>,
}

impl App {
    pub fn new(root_element: Element) -> Self {
        App {
            root_element: root_element.clone(),
            event_loop: EventLoop::new().unwrap(),
            window_state: StateApplication::new(root_element),
        }
    }

    pub async fn run(mut self) {
        let _ = self.event_loop.run_app(&mut self.window_state);
    }
}
