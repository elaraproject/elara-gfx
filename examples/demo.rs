use elara_gfx::gl_info;
use elara_gfx::{GLWindow, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;

struct Handler;

impl WindowHandler for Handler {
    fn on_draw(&self) {
        // All drawing code should be put here
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("Hi OpenGL!")?;
    window.get_context()?;
    gl_info();

    // Event handling
    app.run_loop(window, &Handler);
    Ok(())
}
