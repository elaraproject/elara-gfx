// Demonstrates headless rendering to a PPM image
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, Uniform, VertexArray};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_gfx::types::c_void;
use elara_log::prelude::*;
use std::error::Error;
use std::fs::read_to_string;
use std::process::exit;
use std::time::{Duration, Instant};

const DUMMY_VERTEX_SHADER: &'static str = include_str!("shaders/quad.vert");
const FRAG_SHADER: &'static str = include_str!("shaders/gradient.frag");

struct Handler {
    vao: VertexArray,
    program: Program,
    resolution: (f32, f32),
}

impl Handler {
    fn new(win: &GLWindow) -> Result<Handler, String> {
        let resolution = (win.width() as f32, win.height() as f32);
        let vertex_shader = Shader::new(&DUMMY_VERTEX_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&FRAG_SHADER, gl::FRAGMENT_SHADER)?;
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
        let fbo = FrameBuffer::new()?;
        let color_rbo = RenderBuffer::new()?;

        vao.bind();
        vbo.bind(BufferType::Array);
        vbo.data::<f32>(BufferType::Array, &vertices, gl::STATIC_DRAW);
        ebo.bind(BufferType::ElementArray);
        ebo.data::<i32>(BufferType::ElementArray, &indices, gl::STATIC_DRAW);
        fbo.bind();
        color_rbo.bind();
        
        color_rbo.set_storage(win.width(), win.height());
        fbo.set_renderbuffer(color_rbo);
        fbo.set_color_buffer();
        
        assert!(unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) })
        unsafe {
            glPixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }
        
        vao.vertex_attrib_pointer(0, 3, gl::FLOAT, false, 0);
        vao.enable_vertex_attrib(0);
        vbo.unbind(BufferType::Array);
        vao.unbind();

        Ok(Handler {
            vao,
            program,
            resolution,
        })
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        let width = self.resolution.0;
        let height = self.resolution.1;
        let buffer: Vec<f32> = Vec::with_capacity((width * height * 3.0) as usize);
        unsafe {
            let res_uniform = Uniform::new(self.program.id(), "u_resolution")?;
            gl::Uniform2f(res_uniform.id(), width, height);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::ReadPixels(0, 
                           0, 
                           width as i32, 
                           height as i32, 
                           gl::BGR, 
                           gl::UNSIGNED_BYTE, 
                           buffer.as_ptr() as *mut c_void);
            dbg!(buffer);
            self.vao.unbind();
        }
        exit(0);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("OpenGL shaders")?;
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
