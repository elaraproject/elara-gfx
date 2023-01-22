// Renders a triangle with `elara-gfx`
use elara_gfx::{gl_info, Program, Shader, create_vao, create_vbo};
use elara_gfx::{types::*, GLWindow, HandlerResult, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;
use std::time::{Duration, Instant};

const VERT_SHADER: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER: &str = include_str!("shaders/triangle.frag");

struct Handler {
    vao: GLuint,
    frame_count: u32,
    start_time: Instant,
}

impl Handler {
    fn new(_win: &GLWindow) -> Result<Handler, String> {
        let start_time = Instant::now();
        let frame_count = 0;

        // Perform all onetime CPU operations
        // once here to speed up rendering
        let vao = create_vao();
        let vbo = create_vbo();
        let vertex_shader = Shader::new(&VERT_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&FRAG_SHADER, gl::FRAGMENT_SHADER)?;
        let program = Program::new(&[vertex_shader, fragment_shader])?;
        program.use_program();

        #[rustfmt::skip]
        let vertices = [
            -0.5f32, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0
        ];

        unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

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
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
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
    let render_handler = Handler::new(&window)?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
