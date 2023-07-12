use elara_gfx::{gl_info, Shader, Program, VertexArray, Buffer, BufferType, Color, Uniform, WindowHandler, HandlerResult};
use freetype::Library;
use freetype::face::LoadFlag;
use std::collections::HashMap;
use std::error::Error;
use elara_gfx::GLWindow;
use elara_log::prelude::*;
use elara_gfx::types::c_void;

const VERTEX_SHADER: &'static str = r#"
#version 330 core
layout (location = 0) in vec4 vertex;
out vec2 TexCoord;

void main() {
    gl_Position = vec4(vertex.xy, 0.0, 1.0);
    TexCoord = vertex.zw;
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 330 core
in vec2 TexCoord;
out vec4 FragColor;

uniform sampler2D text;
uniform vec3 textColor;

void main() {
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(text, TexCoord).r);
    FragColor = vec4(textColor, 1.0) * sampled;
}
"#;

fn check_error() {
    let error_code = unsafe { gl::GetError() };
    if error_code != gl::NO_ERROR {
        let error_str = match error_code {
            gl::INVALID_ENUM => "Invalid enum",
            gl::INVALID_VALUE => "Invalid value",
            gl::INVALID_OPERATION => "Invalid operation",
            gl::STACK_OVERFLOW => "Stack overflow",
            gl::STACK_UNDERFLOW => "Stack underflow",
            gl::OUT_OF_MEMORY => "Out of memory",
            gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid framebuffer operation",
            _ => "Unknown GL error"
        };
        error!("{:?} | {}:{}", error_str, file!(), line!());
    } else {
        info!("No errors detected");
    }
}

#[derive(Debug)]
struct Character {
    pub texture_id: i32,
    pub size: (i32, i32),
    pub bearing: (i32, i32),
    pub advance: i32
}

impl Character {
    fn new(texture_id: i32, size: (i32, i32), bearing: (i32, i32), advance: i32) -> Character {
        Character { texture_id, size, bearing, advance }
    }
}

fn render_text(program: &Program, vao: &VertexArray, vbo: &Buffer, characters: &HashMap<char, Character>, text: &str, mut x: f32, y: f32, scale: f32, color: Color) -> Result<(), String> {
    program.use_program();
    let color_uniform = Uniform::new(&program, "textColor")?;
    color_uniform.uniform3f(color.0 as f32 / 255.0, color.1 as f32 / 255.0, color.2 as f32 / 255.0);
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
    }
    vao.bind();
    for c in text.chars() {
        let ch = characters.get(&c).unwrap();
        let xpos = (x + ch.bearing.0 as f32 * scale) / 1200.0;
        let ypos = (y - (ch.size.1 as f32 - ch.bearing.1 as f32) * scale) / 900.0;
        let w = (ch.size.0 as f32 * scale) / 1200.0;
        let h = (ch.size.1 as f32 * scale) / 900.0;
        let vertices: [f32; 24] = [
            xpos,     ypos + h,   0.0_f32, 0.0,            
            xpos,     ypos,       0.0,     1.0,
            xpos + w, ypos,       1.0,     1.0,

            xpos,     ypos + h,   0.0,     0.0,
            xpos + w, ypos,       1.0,     1.0,
            xpos + w, ypos + h,   1.0,     0.0  
        ];
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, ch.texture_id as u32);
            vbo.bind(BufferType::Array);
            vbo.subdata(BufferType::Array, 0, &vertices);
            vbo.unbind(BufferType::Array);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            x += (ch.advance >> 6) as f32 * scale;
        }
    }
    vao.unbind();
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    Ok(())
}

struct Handler {
    program: Program,
    vao: VertexArray,
    vbo: Buffer,
    characters: HashMap<char, Character>
}

impl Handler {
    fn new() -> Result<Handler, String> {
        let vertex_shader = Shader::new(&VERTEX_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&FRAGMENT_SHADER, gl::FRAGMENT_SHADER)?;
        let program = Program::new(&[vertex_shader, fragment_shader])?;

        let vao = VertexArray::new()?;
        let vbo = Buffer::new()?;
        vao.bind();
        vbo.bind(BufferType::Array);
        vbo.data_empty::<f32>(BufferType::Array, 24, gl::DYNAMIC_DRAW);

        let vertex_attrib = 0;
        vao.enable_vertex_attrib(vertex_attrib as u32);
        vao.vertex_attrib_pointer::<f32>(vertex_attrib as u32, 4, gl::FLOAT, false, 4, 0);

        vbo.unbind(BufferType::Array);
        vao.unbind();

        let mut characters: HashMap<char, Character> = HashMap::new();

        if !characters.is_empty() {
            characters.clear();
        }

        let ft = Library::init().unwrap();
        let face = ft.new_face("resources/OpenSans-Regular.ttf", 0).unwrap();
        face.set_pixel_sizes(0, 48).unwrap();
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }
        // Enable blending
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        // Load first 128 characters of ASCII set
        for c in 0..128 as u8 {
            face.load_char(c as usize, LoadFlag::RENDER).unwrap();
            unsafe {
                let mut texture = 0;
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    face.glyph().bitmap().width(),
                    face.glyph().bitmap().rows(),
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    face.glyph().bitmap().buffer().as_ptr() as *const c_void
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                let character = Character::new(
                    texture as i32,
                    (face.glyph().bitmap().width(), face.glyph().bitmap().rows()),
                    (face.glyph().bitmap_left(), face.glyph().bitmap_top()),
                    face.glyph().advance().x as i32
                );

                characters.insert(c as char, character);
            }
        }
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Handler { program, vao, vbo, characters })
    }
}


impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            render_text(&self.program, &self.vao, &self.vbo, &self.characters, "Sample text", 0.0, 0.0, 1.0, Color(255, 255, 255, 1.0)).unwrap();
            render_text(&self.program, &self.vao, &self.vbo, &self.characters, "Rendered by elara-gfx", 255.0, 850.0, 1.0, Color(255, 255, 255, 1.0)).unwrap();
            render_text(&self.program, &self.vao, &self.vbo, &self.characters, "Computer graphics", 255.0, -850.0, 1.0, Color(255, 255, 255, 1.0)).unwrap();
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

