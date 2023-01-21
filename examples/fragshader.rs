use std::env;
use std::fs::read_to_string;
use std::time::Instant;

// Takes almost any fragment shader as input and renders with the shader
// should (eventually) run almost any shader on Shadertoy
use elara_gfx::gfx::*;
use elara_gfx::{Event, EventLoop, GLWindow};
use elara_log::Logger;

fn main() -> GfxResult {
    let mut log = Logger::new();
    log.info("Program starting...");

    // Command-line parsing
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        log.error("Usage: fragshader input.frag")
    }
    let shader_path = &args[0];

    log.info(format!("Attempting to load shader {}", &shader_path).as_str());
    let fs: String = read_to_string(&shader_path)?.parse()?;

    // Begin event loop and create window
    let event_loop = EventLoop::new();
    let window = GLWindow::new(900, 600, "Window 1", true, &event_loop);
    window.init_gl();
    log.info(
        format!(
            "Window dimensions: {} x {}",
            window.width(),
            window.height()
        )
        .as_str(),
    );

    // OpenGL info
    gfxinfo();

    let vertices = vec![
        -1.0, 1.0, 1.0, 0.0, 0.0, // Top-left
        1.0, 1.0, 0.0, 1.0, 0.0, // Top-right
        1.0, -1.0, 0.0, 0.0, 1.0, // Bottom-right
        -1.0, -1.0, 1.0, 1.0, 1.0, // Bottom-left
    ];

    let elements = vec![0, 1, 2, 2, 3, 0];

    let renderer = ElementRenderer::new(vertices, elements, get_dummy_vs(), &fs);
    renderer.attribute("position", 2, 5, null_ptr::<f32>());

    let resolution = [window.width() as f32, window.height() as f32];
    let start_time = Instant::now();

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
                let now = std::time::Instant::now();
                let elapsed_time = now.duration_since(start_time).as_secs_f32();
                renderer.uniform_f32("u_time", &[elapsed_time]);
                renderer.uniform_f32("u_resolution", &resolution);
                renderer.draw(6);
                window.swap_buffers();
                window.make_not_current();
            }
            Event::RedrawRequested(_) => {}
            _ => {}
        }
    });
}
