use elara_gfx::gfx::*;
use elara_gfx::{Event, EventLoop, GLWindow};
use elara_log::Logger;

const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

// Example adapted from https://gist.github.com/newvertex/ac6eda7713b803418645b85e38950308
fn render_triangle() {
    let vertices = [-0.5_f32, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };

    let mut vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vbo) };

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
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(
            vertex_shader,
            1,
            &VERT_SHADER_SRC.as_bytes().as_ptr().cast(),
            &VERT_SHADER_SRC.len().try_into().unwrap(),
        );
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0_i32;
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "Vertex Shader Compile Error: {}",
                String::from_utf8_lossy(&v)
            );
        }
    }

    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(
            fragment_shader,
            1,
            &FRAG_SHADER_SRC.as_bytes().as_ptr().cast(),
            &FRAG_SHADER_SRC.len().try_into().unwrap(),
        );
        gl::CompileShader(fragment_shader);

        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "Fragment Shader Compile Error: {}",
                String::from_utf8_lossy(&v)
            );
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
        gl::ClearColor(0.3, 0.4, 0.6, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    unsafe {
        gl::UseProgram(shader_program);
        gl::BindVertexArray(vao);

        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        gl::BindVertexArray(0);
    }
}

fn main() {
    let mut log = Logger::new();
    let event_loop = EventLoop::new();
    let window = GLWindow::new(900, 600, "Window 1", &event_loop);
    window.init_gl();
    println!("OpenGL Renderer: {}", glGetString(gl::RENDERER));
    println!("OpenGL Version: {}", glGetString(gl::VERSION));
    println!(
        "GLSL Version: {}",
        glGetString(gl::SHADING_LANGUAGE_VERSION)
    );
    // Event handling
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait(); //set_poll() in actual case

        match event {
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                log.info("Close request received, exiting...");
                control_flow.set_exit();
            }
            Event::MainEventsCleared => {
                window.render(&render_triangle);
            }
            _ => {}
        }
    });
}
