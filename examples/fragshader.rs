// Takes almost any fragment shader as input and renders with the shader
// should (eventually) run almost any shader on Shadertoy
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, Uniform, VertexArray};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;
use std::fs::read_to_string;
use std::process::exit;
use std::time::{Duration, Instant};

const DUMMY_VERTEX_SHADER: &'static str = include_str!("shaders/quad.vert");

struct Handler {
    vao: VertexArray,
    program: Program,
    resolution: (f32, f32),
    frame_count: u32,
    start_time: Instant,
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

impl Handler {
    fn new(win: &GLWindow, shader_src: String) -> Result<Handler, String> {
        let start_time = Instant::now();
        let frame_count = 0;
        let resolution = (win.width() as f32, win.height() as f32);
        let vertex_shader = Shader::new(&DUMMY_VERTEX_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&shader_src, gl::FRAGMENT_SHADER)?;
        let program = Program::new(&[vertex_shader, fragment_shader])?;
        program.use_program();

        // Render 1 fullscreen quad for shaders
        #[rustfmt::skip]
        let vertices = [
            1.0_f32, 1.0, 0.0, // top right
            1.0, -1.0, 0.0, // bottom right
            -1.0, -1.0, 0.0, // bottom left
            -1.0, 1.0, 0.0, // top left
        ];

        #[rustfmt::skip]
        let indices = [
            0, 1, 3,
            1, 2, 3
        ];

        let vao = VertexArray::new()?;
        let vbo = Buffer::new()?;
        let ebo = Buffer::new()?;

        vao.bind();
        vbo.bind(BufferType::Array);
        vbo.data::<f32>(BufferType::Array, &vertices, gl::STATIC_DRAW);
        ebo.bind(BufferType::ElementArray);
        ebo.data::<i32>(BufferType::ElementArray, &indices, gl::STATIC_DRAW);
        vao.vertex_attrib_pointer(0, 3, gl::FLOAT, false, 0);
        vao.enable_vertex_attrib(0);
        vbo.unbind(BufferType::Array);
        vao.unbind();

        Ok(Handler {
            vao,
            program,
            resolution,
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

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            let res_uniform = Uniform::new(self.program.id(), "u_resolution")?;
            let time_uniform = Uniform::new(self.program.id(), "u_time")?;

            let now = std::time::Instant::now();
            let elapsed_time = now.duration_since(self.start_time).as_secs_f32();
            
            time_uniform.uniform1f(elapsed_time);
            res_uniform.uniform2f(self.resolution.0, self.resolution.1);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
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

    // Command-line parsing
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 1 {
        error!("Usage: fragshader input.frag");
        exit(1);
    }
    let shader_path = &args[0];

    info!("Attempting to load shader {}", &shader_path);
    let shader_src: String = read_to_string(&shader_path)?.parse()?;

    let (app, window) = GLWindow::new_with_title("OpenGL shaders")?;
    window.get_context()?;
    gl_info();

    // Run all OpenGL calls that only
    // needs to be run once in advance
    // of rendering to improve performance
    let render_handler = Handler::new(&window, shader_src)?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
