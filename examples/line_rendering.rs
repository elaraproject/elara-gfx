use elara_gfx::{gl_info, WindowHandler, GLWindow};
use elara_gfx::canvas::{LineRenderer, Color};
use std::error::Error;
use elara_log::prelude::*;

struct Handler {
    renderer: LineRenderer
}

impl Handler {
    fn new() -> Result<Handler, String> {
        let renderer = LineRenderer::new()?;
        Ok(Handler{ renderer })
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> Result<(), String> {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
       self.renderer.render_horizontal_line(0, 500, 1200, 4.0, Color(255, 255, 255, 1.0))?;
       self.renderer.render_vertical_line(500, 0, 900, 4.0, Color(0, 255, 255, 1.0))?;
       Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("Rendering lines!")?;
    window.get_context()?;
    gl_info();
    
    let render_handler = Handler::new()?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}