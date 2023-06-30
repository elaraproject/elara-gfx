// This demo shows how to draw polygons and other shapes
// using elara-gfx
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
        canvas.set_background(Color(255, 255, 255, 1.0));
        canvas.add_rect(-0.5, 0.0, 0.8, 0.5, Color(255, 0, 0, 1.0));
        canvas.add_polygon(0.0, 0.0, 0.3, 6, Color(255, 0, 255, 1.0));
        canvas.add_rect(0.1, 0.3, 0.4, 0.3, Color(0, 255, 0, 1.0));
        canvas.add_circle(0.0, -0.2, 0.2, Color(0, 255, 255, 1.0));
        canvas.add_line(vec![[0.0, 0.9], [0.2, 0.8], [0.5, 0.6], [0.8, 0.5], [0.9, 0.3]], 2.0, Color(0, 122, 122, 1.0), false);
        canvas.add_quad([0.0, -0.5], [0.7, -0.5], [0.5, -0.8], [0.0, -0.6], Color(76, 102, 122, 1.0));
        canvas.add_heart(0.8, 0.0, 0.5, Color(255, 20, 193, 1.0));
        canvas.add_text(-0.5, 0.0, "Hello World!", 16.0);
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
    let render_handler = CanvasHandler::new(&window, canvas, false)?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
