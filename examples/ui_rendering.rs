// This demo renders a basic UI with elara-gfx
use elara_gfx::{gl_info, Canvas, Color, Draw, CanvasHandler, GLWindow};
use elara_log::prelude::*;
use std::error::Error;

struct CanvasContext;

impl CanvasContext {
    fn new() -> CanvasContext {
        CanvasContext {}
    }
}

impl Draw for CanvasContext {
    fn draw(&mut self, win: &GLWindow) -> Result<Canvas, String> {
        let mut canvas = Canvas::new(&win);
        canvas.set_background(Color(19, 19, 20, 1.0));
        canvas.add_rect(-0.3, -0.9, 0.7, 1.8, Color(26, 28, 32, 1.0));
        canvas.add_text(0.0, 0.0, "Hello Serendipitous World!", 11.0, true);
        Ok(canvas)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("OpenGL polygons")?;
    window.get_context()?;
    gl_info();
    
    let canvas = CanvasContext::new();
    // Run all OpenGL calls that only
    // needs to be run once in advance
    // of rendering to improve performance
    let render_handler = CanvasHandler::new(&window, canvas, true)?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
