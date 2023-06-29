// This demo shows how to draw polygons and other shapes
// using elara-gfx
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, Uniform, VertexArray, PixelArray, Texture2D, Canvas, Color, ATLAS_IMG};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;

const VERT_SHADER: &str = include_str!("shaders/polygon.vert"); 
const FRAG_SHADER: &str = include_str!("shaders/polygon.frag");

struct Handler {
    vao: VertexArray,
    vertex_num: usize,
    background: Color,
    program: Program,
    aspect_ratio: f32,
    texture: Texture2D
}

impl Handler {
    fn new(win: &GLWindow) -> Result<Handler, String> {
        // Draw code here
        let mut canvas = Canvas::new(&win);
        let img = PixelArray::load_png(ATLAS_IMG).unwrap();
        canvas.set_background(Color(1.0, 1.0, 1.0, 1.0));
        canvas.add_rect(-0.5, 0.0, 0.8, 0.5, Color(1.0, 0.0, 0.0, 1.0));
        canvas.add_polygon(0.0, 0.0, 0.3, 6, Color(1.0, 0.0, 1.0, 1.0));
        canvas.add_rect(0.1, 0.3, 0.4, 0.3, Color(0.0, 1.0, 0.0, 1.0));
        canvas.add_circle(0.0, -0.2, 0.2, Color(0.0, 1.0, 1.0, 1.0));
        canvas.add_line(vec![[0.0, 0.9], [0.2, 0.8], [0.5, 0.6], [0.8, 0.5], [0.9, 0.3]], 2.0, Color(0.0, 0.5, 0.5, 1.0), false);
        canvas.add_quad([0.0, -0.5], [0.7, -0.5], [0.5, -0.8], [0.0, -0.6], Color(0.3, 0.4, 0.5, 1.0));
        canvas.add_heart(0.8, 0.0, 0.5, Color(1.0, 0.08, 0.76, 1.0));
        canvas.add_text(-0.5, 0.0, "Hello World!", 16.0);

        // End draw code
        let vertices = &canvas.to_vertices();
        let vertex_num = canvas.len();
        let background = canvas.background();
        let aspect_ratio = win.height() as f32 / win.width() as f32;

        let texture = Texture2D::new()?;
        texture.bind();
        texture.parameter_2d(gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        texture.parameter_2d(gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        texture.enable_alpha_blend();

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

        vao.vertex_attrib_pointer::<f32>(pos_attrib as u32, 2, gl::FLOAT, false, 8, 0);
        vao.vertex_attrib_pointer::<f32>(col_attrib as u32, 4, gl::FLOAT, false, 8, 2);
        vao.vertex_attrib_pointer::<f32>(tex_coord_attrib as u32, 2, gl::FLOAT, false, 8, 6);

        vao.unbind();
        vbo.unbind(BufferType::Array);

        Ok(Handler {
            vao,
            vertex_num,
            background,
            program,
            aspect_ratio,
            texture
        })
    }
}


impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            let aspect_ratio_uniform = Uniform::new(&self.program, "aspect_ratio")?;
            aspect_ratio_uniform.uniform1f(self.aspect_ratio);
            gl::ClearColor(self.background.0, self.background.1, self.background.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.texture.bind();
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num as i32);
            self.vao.unbind();
            self.texture.unbind();
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::new().init().unwrap();
    info!("Starting logging...");

    let (app, window) = GLWindow::new_with_title("OpenGL polygons")?;
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
