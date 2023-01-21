use std::path::PathBuf;

use effi::c_void;
// Demonstrates headless rendering to a
// PPM image
use elara_gfx::gfx::*;
use elara_gfx::{Event, EventLoop, GLWindow};
use elara_log::Logger;

const FRAG_SHADER_SRC: &str = include_str!("shaders/gradient.frag");
const HEADLESS_ENABLED: bool = false;

fn main() -> GfxResult {
    let mut log = Logger::new();
    log.info("Program starting...");

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

    let renderer = ElementRenderer::new(vertices, elements, get_dummy_vs(), &FRAG_SHADER_SRC);
    renderer.attribute("position", 2, 5, null_ptr::<f32>());

    let resolution = [window.width() as f32, window.height() as f32];

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
                renderer.uniform_f32("u_resolution", &resolution);
                renderer.draw(6);
                if HEADLESS_ENABLED {
                    let (width, height): (i32, i32) = (900, 600);
                    let pixels: Vec<u8> =
                        Vec::with_capacity((width * height * 4 as i32).try_into().unwrap());
                    unsafe {
                        gl::ReadPixels(
                            0,
                            0,
                            width,
                            height,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            pixels.as_slice().as_ptr() as *mut c_void,
                        );
                    }

                    let mut image = PixelArray::new(width, height);
                    image.add_data(&pixels);
                    let image_file: PathBuf = PathBuf::from("OpenGL-rendering-result.ppm");
                    image.save_as_ppm(image_file);
                }
                window.swap_buffers();
                window.make_not_current();
            }
            Event::RedrawRequested(_) => {}
            _ => {}
        }
    });
}
