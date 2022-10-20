use elara_gfx::gfx::*;
use elara_gfx::{Event, EventLoop, GLWindow};
use elara_log::Logger;

fn main() {
    let mut log = Logger::new();
    let event_loop = EventLoop::new();
    let window = GLWindow::new(900, 600, "Window 1", true, &event_loop);
    window.init_gl();
    gfxinfo();
    log.info("Starting rendering...");
    // Event handling
    event_loop.run(move |event, _, control_flow| {

        match event {
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                log.info("Close request received, exiting...");
                control_flow.set_exit();
            }
            Event::MainEventsCleared => {
                // Render function
                window.make_current();
                window.clear(0.1, 0.1, 0.1, 1.0);
                window.swap_buffers();
                window.make_not_current();
            }
            Event::RedrawRequested(_) => {
            }
            _ => {}
        }
    });
}
