// Renders a triangle with `elara-gfx`
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, VertexArray};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;
use std::time::{Duration, Instant};

const VERT_SHADER: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER: &str = include_str!("shaders/triangle.frag");

struct Handler {
    vao: VertexArray,
    frame_count: u32,
    start_time: Instant,
}

impl Handler {
    fn new() -> Result<Handler, String> {
        let start_time = Instant::now();
        let frame_count = 0;

        // Perform all onetime CPU operations
        // once here to speed up rendering
        #[rustfmt::skip]
        let vertices = [
            -0.5f32, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0
        ];

        let vao = VertexArray::new()?;
        vao.bind();

        let vbo = Buffer::new()?;
        vbo.bind(BufferType::Array);
        vbo.data::<f32>(BufferType::Array, &vertices, gl::STATIC_DRAW);
        vao.vertex_attrib_pointer(0, 3, gl::FLOAT, false, 0);
        vao.enable_vertex_attrib(0);

        vao.unbind();
        vbo.unbind(BufferType::Array);

        let vertex_shader = Shader::new(&VERT_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&FRAG_SHADER, gl::FRAGMENT_SHADER)?;
        let program = Program::new(&[vertex_shader, fragment_shader])?;
        program.use_program();

        Ok(Handler {
            vao,
            frame_count,
            start_time,
        })
    }
    fn current_frame(&self) -> u32 {
        self.frame_count
    }
    fn add_frame(&mut self) {
        self.frame_count += 1
    }
    fn get_elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }
}

fn show_render_stats(frame_interval: u32, current_frame: u32, frame_render_time: Duration) {
    // Shows render stats once per <frame_interval> frames
    if current_frame % frame_interval == 0 {
        debug!(
            "Frame {} average render time {:?}",
            current_frame, frame_render_time
        );
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            self.vao.unbind();
        }
        self.add_frame();
        let current_frame = self.current_frame();
        let frame_render_time = self.get_elapsed_time() / current_frame;
        // Avoid showing render time counter too often so we show only once
        // per every few hundred frames
        show_render_stats(500, current_frame, frame_render_time);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("OpenGL triangles")?;
    window.get_context()?;
    gl_info();

    // Run all OpenGL calls that only
    // needs to be run once in advance
    // of rendering to improve performance
    let render_handler = Handler::new()?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
