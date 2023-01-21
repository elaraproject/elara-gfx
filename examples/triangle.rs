// Renders a triangle with `elara-gfx`
use elara_gfx::{gl_info, to_cstr};
use elara_gfx::{types::*, GLWindow, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;
use std::time::{Duration, Instant};

const VERT_SHADER: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER: &str = include_str!("shaders/triangle.frag");

struct Handler {
    frame_count: u32,
    start_time: Instant,
    dims: (i32, i32),
}

type Vertex = [f32; 3];

// fn create_vao() -> u32 {
//     let mut vao = 0;
//     unsafe { gl::GenVertexArrays(1, &mut vao) };
//     vao
// }

// fn create_vbo() -> u32 {
//     let mut vbo = 0;
//     unsafe { gl::GenBuffers(1, &mut vbo) };
//     vbo
// }

// fn create_shader(shader_src: &str, shader_type: GLenum) -> GLuint {
//     let shader = unsafe { gl::CreateShader(shader_type) };
//     unsafe {
//         gl::ShaderSource(
//             shader,
//             1,
//             &shader_src.as_bytes().as_ptr().cast(),
//             &shader_src.len().try_into().unwrap(),
//         );
//         gl::CompileShader(shader);

//         let mut success = 0;
//         gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
//         if success == 0 {
//             let mut log_len = 0_i32;
//             let mut v: Vec<u8> = Vec::with_capacity(1024);
//             gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
//             v.set_len(log_len.try_into().unwrap());
//             error!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
//         }
//     }
//     shader
// }

// fn create_program(vs: GLuint, fs: GLuint) -> GLuint {
//     let shader_program = unsafe { gl::CreateProgram() };
//     unsafe {
//         gl::AttachShader(shader_program, vs);
//         gl::AttachShader(shader_program, fs);
//         gl::LinkProgram(shader_program);

//         let mut success = 0;
//         gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
//         if success == 0 {
//             let mut v: Vec<u8> = Vec::with_capacity(1024);
//             let mut log_len = 0_i32;
//             gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
//             v.set_len(log_len.try_into().unwrap());
//             panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
//         }

//         gl::DetachShader(shader_program, vs);
//         gl::DetachShader(shader_program, fs);
//         gl::DeleteShader(vs);
//         gl::DeleteShader(fs);
//     }
//     shader_program
// }

impl Handler {
    fn new(win: &GLWindow) -> Handler {
        let dims = (win.width(), win.height());
        let start_time = Instant::now();
        let frame_count = 0;
        Handler {
            frame_count,
            start_time,
            dims,
        }
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
            "Frame {} rendered in {:?}",
            current_frame, frame_render_time
        );
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) {
        let vertices: Vec<f32> = vec![
            0.0f32, 0.5,
            0.5, -0.5,
            -0.5, -0.5
        ];
        
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);
        }
        
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        }

        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
        
        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        assert_ne!(vertex_shader, 0);
        
        unsafe {
            gl::ShaderSource(
                vertex_shader,
                1,
                &(VERT_SHADER.as_bytes().as_ptr().cast()),
                &(VERT_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(vertex_shader);
        }
        
        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        assert_ne!(fragment_shader, 0);
        
        unsafe {
            gl::ShaderSource(
                fragment_shader,
                1,
                &(FRAG_SHADER.as_bytes().as_ptr().cast()),
                &(FRAG_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(fragment_shader);
        }
        
        let shader_program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::BindFragDataLocation(shader_program, 0, to_cstr("outColor"));
            gl::LinkProgram(shader_program);
            gl::UseProgram(shader_program);
        }
        
        let pos_attrib = unsafe { gl::GetAttribLocation(shader_program, to_cstr("position")) };
        unsafe {
            gl::VertexAttribPointer(
                pos_attrib as u32,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                0 as *const _,
            );
            gl::EnableVertexAttribArray(pos_attrib as u32);
        }
        
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            assert_ne!(vao, 0);
        }
        unsafe {
            gl::BindVertexArray(vao);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        self.add_frame();
        let current_frame = self.current_frame();
        let frame_render_time = self.get_elapsed_time() / current_frame;
        // Avoid showing render time counter too often so we show only once
        // per every few hundred frames
        show_render_stats(500, current_frame, frame_render_time);
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
    let render_handler = Handler::new(&window);

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
