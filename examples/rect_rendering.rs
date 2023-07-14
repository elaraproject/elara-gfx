use elara_gfx::{gl_info, WindowHandler, HandlerResult, GLWindow};
use std::error::Error;
use elara_log::prelude::*;
use elara_gfx::canvas::{RectRenderer, RectStyle};

struct Handler {
    renderer: RectRenderer
}

impl Handler {
    fn new() -> Result<Handler, String> {
        let renderer = RectRenderer::new()?;
        Ok(Handler { renderer })
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.renderer.render_rect(RectStyle::new())?;
            self.renderer.render_rect(RectStyle::new().position(300.0, 400.0).border_radius(10.0))?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("Hi OpenGL!")?;
    window.get_context()?;
    gl_info();
    
    let render_handler = Handler::new()?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}