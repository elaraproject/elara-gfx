// Renders a triangle with `elara-gfx`
use elara_gfx::gfx::*;
use elara_gfx::{Event, EventLoop, GLWindow};
use elara_log::Logger;

const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

fn main() -> GfxResult {
    let mut log = Logger::new();
    log.info("Program starting...");

    // Begin event loop and create window
    let event_loop = EventLoop::new();
    let window = GLWindow::new(900, 600, "Window 1", true, &event_loop);
    window.init_gl();

    // OpenGL info
    gfxinfo();

    let vertices = vec![
        0.0_f32, 0.5, 1.0, 0.0, 0.0, // Vertex 1: Red
        0.5, -0.5, 0.0, 1.0, 0.0, // Vertex 2: Green
        -0.5, -0.5, 0.0, 0.0, 1.0 // Vertex 3: Blue
    ];

    let renderer = DefaultRenderer::new(vertices, &VERT_SHADER_SRC, &FRAG_SHADER_SRC);
    renderer.attribute("position", 2, 5, null_ptr::<f32>());
    renderer.attribute("color", 3, 5, sized_ptr::<f32>(2));

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
                renderer.bg(0.1, 0.1, 0.1, 1.0);
                renderer.draw_default();
                window.swap_buffers();
                window.make_not_current();
            }
            Event::RedrawRequested(_) => {
            }
            _ => {
            }
        }
    });
}
