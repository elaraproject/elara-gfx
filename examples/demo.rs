use elara_gfx::gfx::*;
use elara_gfx::{Event, EventLoop, GLWindow};
use elara_log::Logger;

// const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
// const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

fn main() {
    let mut log = Logger::new();
    let event_loop = EventLoop::new();
    let window = GLWindow::new(900, 600, "Window 1", &event_loop);
    window.init_gl();
    println!("OpenGL Renderer: {}", glGetString(gl::RENDERER));
    println!("OpenGL Version: {}", glGetString(gl::VERSION));
    println!(
        "GLSL Version: {}",
        glGetString(gl::SHADING_LANGUAGE_VERSION)
    );
    // Event handling
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait(); //set_poll() in actual case

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
            }
            Event::RedrawRequested(_) => {
                window.redraw();
            }
            _ => {}
        }
    });
}
