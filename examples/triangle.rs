// Renders a triangle with `elara-gfx`
use elara_gfx::gl_info;
use elara_gfx::{GLWindow, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;

const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

struct Handler;

impl WindowHandler for Handler {
    fn on_draw(&self) {
        // Code here really only needs to be called once
        // ---------
        let vertices = vec![
            -0.5f32, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
        ];
        
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };

        let mut vbo = 0;
        unsafe { gl::GenBuffers(1, &mut vbo) };
        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        unsafe {
            gl::ShaderSource(vertex_shader, 1, &VERT_SHADER_SRC.as_bytes().as_ptr().cast(), &VERT_SHADER_SRC.len().try_into().unwrap());
            gl::CompileShader(vertex_shader);
            
            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut log_len = 0_i32;
                // gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut log_len);
                // let mut v: Vec<u8> = Vec::with_capacity(log_len as usize);
                // gl::GetShaderInfoLog(vertex_shader, log_len, &mut log_len, v.as_mut_ptr().cast());
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                error!("Vertex Shader Compile Error: {}", String::from_utf8_lossy(&v));
            }
        }

        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        unsafe {
            gl::ShaderSource(fragment_shader, 1, &FRAG_SHADER_SRC.as_bytes().as_ptr().cast(), &FRAG_SHADER_SRC.len().try_into().unwrap());
            gl::CompileShader(fragment_shader);
            
            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                error!("Fragment Shader Compile Error: {}", String::from_utf8_lossy(&v));
            }
        }

        let shader_program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }

            gl::DetachShader(shader_program, vertex_shader);
            gl::DetachShader(shader_program, fragment_shader);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
         unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&vertices) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, 0 as *const _);
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        
        // -----------------
        
         unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindVertexArray(0);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("OpenGL triangles")?;
    window.get_context()?;
    gl_info();

    // Event handling
    app.run_loop(window, &Handler);
    Ok(())
}
