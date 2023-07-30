// This demo renders a basic UI with elara-gfx
use elara_gfx::{gl_info, GLWindow, WindowHandler};
use elara_gfx::canvas::{clear_color, TextRenderer, RectRenderer, RectStyle, LineRenderer, Color};
use elara_log::prelude::*;
use std::error::Error;
use std::time::Instant;

struct Handler {
    text_renderer: TextRenderer,
    rect_renderer: RectRenderer,
    line_renderer: LineRenderer
}

impl Handler {
    fn new(win: &GLWindow) -> Result<Handler, String> {
        let mut text_renderer = TextRenderer::new(win)?;
        text_renderer.load("resources/OpenSans-Regular.ttf", 40);
        let rect_renderer = RectRenderer::new()?;
        let line_renderer = LineRenderer::new()?;
        Ok(Handler{ text_renderer, rect_renderer, line_renderer })
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> Result<(), String> {
        let now = Instant::now();
        clear_color(Color(19, 19, 20, 1.0));
        self.rect_renderer.render_rect(RectStyle::new()
            .dims(420, 750)
            .rect_color(26, 28, 32)
            .position(300.0, 300.0)
            .border_color(35, 36, 40)
            .border_thickness(4.0)
            .border_radius(10.0))?;
        self.text_renderer.render_text("View Options", 330, 980, 1.0, Color(107, 110, 120, 1.0))?;
        self.text_renderer.render_text("Point cloud", 330, 920, 1.0, Color(255, 255, 255, 1.0))?;
        self.text_renderer.render_text("Point size", 330, 870, 1.0, Color(107, 110, 120, 1.0))?;
        self.rect_renderer.render_rect(RectStyle::new()
            .dims(130, 40)
            .rect_color(42, 46, 53)
            .position(500.0, 860.0)
            .border_radius(5.0))?;
        self.rect_renderer.render_rect(RectStyle::new()
            .dims(55, 40)
            .rect_color(0, 126, 216)
            .position(500.0, 860.0)
            .border_radius(5.0))?;
        self.rect_renderer.render_rect(RectStyle::new()
            .dims(55, 40)
            .rect_color(42, 46, 53)
            .position(640.0, 860.0)
            .border_radius(5.0))?;
        self.text_renderer.render_text("9", 660, 870, 1.0, Color(107, 110, 120, 1.0))?;
        self.line_renderer.render_horizontal_line(330, 840, 370, 1.0, Color(42, 46, 53, 1.0))?;
        self.text_renderer.render_text("Coordinate frames", 330, 790, 1.0, Color(255, 255, 255, 1.0))?;
        self.text_renderer.render_text("+", 660, 790, 1.0, Color(255, 255, 255, 1.0))?;
        self.line_renderer.render_horizontal_line(330, 760, 370, 1.0, Color(42, 46, 53, 1.0))?;
        self.text_renderer.render_text("CAD matches", 330, 710, 1.0, Color(255, 255, 255, 1.0))?;
        self.text_renderer.render_text("+", 660, 710, 1.0, Color(255, 255, 255, 1.0))?;
        self.line_renderer.render_horizontal_line(330, 680, 370, 1.0, Color(42, 46, 53, 1.0))?;
        self.text_renderer.render_text("Grasps", 330, 630, 1.0, Color(255, 255, 255, 1.0))?;
        self.text_renderer.render_text("+", 660, 630, 1.0, Color(255, 255, 255, 1.0))?;
        self.text_renderer.render_text("Returned", 330, 580, 1.0, Color(107, 110, 120, 1.0))?;
        self.rect_renderer.render_rect(RectStyle::new()
            .dims(90, 40)
            .rect_color(0, 126, 216)
            .position(500.0, 570.0)
            .border_radius(5.0))?;
        self.text_renderer.render_text("Show", 520, 580, 1.0, Color(255, 255, 255, 1.0))?;
        self.rect_renderer.render_rect(RectStyle::new()
            .dims(90, 40)
            .rect_color(42, 46, 53)
            .position(600.0, 570.0)
            .border_radius(5.0))?;
        self.text_renderer.render_text("Hide", 620, 580, 1.0, Color(107, 110, 120, 1.0))?;
        self.text_renderer.render_text("Returned", 330, 530, 1.0, Color(107, 110, 120, 1.0))?;
        self.rect_renderer.render_rect(RectStyle::new()
            .dims(90, 40)
            .rect_color(42, 46, 53)
            .position(500.0, 520.0)
            .border_radius(5.0))?;
        self.text_renderer.render_text("Show", 520, 530, 1.0, Color(107, 110, 120, 1.0))?;
        self.rect_renderer.render_rect(RectStyle::new()
            .dims(90, 40)
            .rect_color(0, 126, 216)
            .position(600.0, 520.0)
            .border_radius(5.0))?;
        self.text_renderer.render_text("Hide", 620, 530, 1.0, Color(255, 255, 255, 1.0))?;
        self.line_renderer.render_horizontal_line(330, 500, 370, 1.0, Color(42, 46, 53, 1.0))?;
        self.text_renderer.render_text("Gripper", 330, 460, 1.0, Color(255, 255, 255, 1.0))?;
        self.text_renderer.render_text("+", 660, 460, 1.0, Color(255, 255, 255, 1.0))?;
        self.line_renderer.render_horizontal_line(330, 430, 370, 1.0, Color(42, 46, 53, 1.0))?;
        info!("Render time is {:?}", now.elapsed());
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_sized("UI rendering demo", 1600, 1200)?;
    window.get_context()?;
    gl_info();
    
    // Run all OpenGL calls that only
    // needs to be run once in advance
    // of rendering to improve performance
    let render_handler = Handler::new(&window)?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
