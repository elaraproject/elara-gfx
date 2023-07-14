use elara_gfx::{gl_info, Shader, Program, VertexArray, Buffer, BufferType, Uniform, WindowHandler, HandlerResult};
use std::error::Error;
use elara_gfx::GLWindow;
use elara_log::prelude::*;
use elara_gfx::canvas::{TextRenderer, Color};

struct Handler {
    renderer: TextRenderer
}

impl Handler {
    fn new() -> Result<Handler, String> {
        let mut renderer = TextRenderer::new()?;
        renderer.load("resources/OpenSans-Regular.ttf", 48);
        Ok(Handler { renderer })
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.renderer.render_text("Hello World from OpenGL!", 0.0, 0.0, 1.0, Color(255, 255, 255, 1.0)).unwrap();
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

