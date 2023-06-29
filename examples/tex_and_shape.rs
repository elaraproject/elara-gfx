// This demo showcases how to render a texture on top of a shape in elara-gfx
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, VertexArray, PixelArray, Uniform, Texture2D};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;

const VERT_SHADER: &str = include_str!("shaders/tex_and_shape.vert");
const FRAG_SHADER: &str = include_str!("shaders/tex_and_shape.frag");
const IMG_PATH: &str = "resources/text_white.png";

struct Handler {
    aspect_ratio: f32,
    vao: VertexArray,
    texture: Texture2D,
    program: Program
}
impl Handler {
    fn new(win: &GLWindow) -> Result<Handler, String> {

        let aspect_ratio = win.height() as f32 / win.width() as f32;
        
        let vertices: [f32; 84] = [
             // positions  // colors        // texture coords
             1.0,  1.0,    0.27, 0.25, 0.26,   1.0, 1.0, // top right
             1.0,  0.0,    0.27, 0.25, 0.26,   1.0, 1.0, // bottom right
             0.0,  1.0,    0.27, 0.25, 0.26,   1.0, 1.0, // top left
             1.0,  0.0,    0.27, 0.25, 0.26,   1.0, 1.0, // bottom right
             0.0,  0.0,    0.27, 0.25, 0.26,   1.0, 1.0, // bottom left
            -0.0,  1.0,    0.27, 0.25, 0.26,   1.0, 1.0,  // top left

             0.5,  0.5,    0.13, 0.15, 0.16,   1.0, 1.0, // top right
             0.5, -0.5,    0.13, 0.15, 0.16,   1.0, 0.0, // bottom right
            -0.5,  0.5,    0.13, 0.15, 0.16,   0.0, 1.0, // top left
             0.5, -0.5,    0.13, 0.15, 0.16,   1.0, 0.0, // bottom right
            -0.5, -0.5,    0.13, 0.15, 0.16,   0.0, 0.0, // bottom left
            -0.5,  0.5,    0.13, 0.15, 0.16,   0.0, 1.0,  // top left
        ];

        let texture = Texture2D::new()?;
        texture.bind();

        texture.parameter_2d(gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        texture.parameter_2d(gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        let mut img = PixelArray::load_png_from_path(IMG_PATH).unwrap();
        img.flipv();
        texture.set_image_2d(img);
        texture.generate_mipmap();

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
        let col_attrib = vao.get_attrib_location(&program, "vertex_color");
        let tex_coord_attrib = vao.get_attrib_location(&program, "tex_coord");

        vao.enable_vertex_attrib(pos_attrib as u32);
        vao.enable_vertex_attrib(col_attrib as u32);
        vao.enable_vertex_attrib(tex_coord_attrib as u32);

        vao.vertex_attrib_pointer::<f32>(pos_attrib as u32, 2, gl::FLOAT, false, 7, 0);
        vao.vertex_attrib_pointer::<f32>(col_attrib as u32, 3, gl::FLOAT, false, 7, 2);
        vao.vertex_attrib_pointer::<f32>(tex_coord_attrib as u32, 2, gl::FLOAT, false, 7, 5);

        Ok(Handler {aspect_ratio, vao, texture, program})

    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            let aspect_ratio_uniform = Uniform::new(&self.program, "aspect_ratio")?;
            aspect_ratio_uniform.uniform1f(self.aspect_ratio);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.texture.bind();
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 12);
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
    
    let render_handler = Handler::new(&window)?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}

