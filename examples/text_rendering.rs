use elara_gfx::{gl_info, WindowHandler, HandlerResult};
use std::error::Error;
use elara_gfx::GLWindow;
use elara_log::prelude::*;
use elara_gfx::canvas::{TextRenderer, Color};

struct Handler {
    renderer: TextRenderer
}

impl Handler {
    fn new(win: &GLWindow) -> Result<Handler, String> {
        let mut renderer = TextRenderer::new(win)?;
        renderer.load("resources/OpenSans-Regular.ttf", 48);
        Ok(Handler { renderer })
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.renderer.render_text("Bottom left", 0, 0, 1.0, Color(255, 255, 255, 1.0))?;
            self.renderer.render_text("Center", 600, 450, 1.0, Color(255, 255, 255, 1.0))?;
            self.renderer.render_text("Top right", 1120, 880, 1.0, Color(255, 255, 255, 1.0))?;
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

    let render_handler = Handler::new(&window)?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}

