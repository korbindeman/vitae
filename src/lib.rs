mod immediate_ui;
mod renderer_wgpu;
mod window;

use window::StateApplication;
use winit::event_loop::EventLoop;

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let mut window_state = StateApplication::new();

    let _ = event_loop.run_app(&mut window_state);
}
