// demonstrates how to draw a basic image
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, Uniform, VertexArray, Texture2D, PixelArray};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_gfx::types::GLuint;
use elara_log::prelude::*;
use elara_log::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::borrow::Cow;
use std::path::{PathBuf, Path};

const VERT_SHADER: &str = include_str!("shaders/image.vert");
const FRAG_SHADER: &str = include_str!("shaders/image.frag");
const IMG_PATH: &str = "resources/text_flipped.png";

struct Handler {
    vao: VertexArray,
    texture: Texture2D
}

impl Handler {
    fn new() -> Result<Handler, String> {
        let vertices: [f32; 24] = [
             // positions // texture coords
             0.5,  0.5,   1.0, 1.0, // top right
             0.5, -0.5,   1.0, 0.0, // bottom right
            -0.5,  0.5,   0.0, 1.0, // top left
             0.5, -0.5,   1.0, 0.0, // bottom right
            -0.5, -0.5,   0.0, 0.0, // bottom left
            -0.5,  0.5,   0.0, 1.0  // top left
        ];

        let vao = VertexArray::new()?;
        vao.bind();

        let vbo = Buffer::new()?;
        vbo.bind(BufferType::Array);
        vbo.data::<f32>(BufferType::Array, &vertices, gl::STATIC_DRAW);
        
        let vertex_shader = Shader::new(&VERT_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&FRAG_SHADER, gl::FRAGMENT_SHADER)?;
        let program = Program::new(&[vertex_shader, fragment_shader])?;
        program.use_program();
        
        let pos_attrib = vao.get_attrib_location(&program, "position");
        vao.enable_vertex_attrib(pos_attrib as u32);
        vao.vertex_attrib_pointer::<f32>(pos_attrib as u32, 2, gl::FLOAT, false, 4, 0);
        
        let tex_coord_attrib = vao.get_attrib_location(&program, "tex_coord");
        vao.enable_vertex_attrib(tex_coord_attrib as u32);
        vao.vertex_attrib_pointer::<f32>(tex_coord_attrib as u32, 2, gl::FLOAT, false, 4, 2);
        
        let texture = Texture2D::new()?;
        texture.bind();

        // Texture parameters
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            // Enable blending
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        
        let img = PixelArray::load_png(IMG_PATH).unwrap();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           img.width as i32,
                           img.height as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           img.data().as_ptr() as *const u8 as *const elara_gfx::types::c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Ok(Handler { vao, texture })
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> Result<(), String> {
        // All drawing code should be put here
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.texture.bind();
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            self.vao.unbind();
            self.texture.unbind();
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("Hi OpenGL!")?;
    window.get_context()?;
    gl_info();
    
    let render_handler = Handler::new()?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
