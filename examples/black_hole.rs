// demonstrates advanced raytracing
// see: https://medium.com/@cadenmarinozzi/simulating-a-schwarzschild-black-hole-part-1-the-background-and-raytracer-7de436a56b7e
// space bg image credit: https://svs.gsfc.nasa.gov/4851/
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, VertexArray, Texture2D, PixelArray, Uniform};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;

const VERT_SHADER: &str = include_str!("shaders/blackhole.vert");
const FRAG_SHADER: &str = include_str!("shaders/blackhole.frag");
const IMG_PATH: &str = "resources/starmap_g4k.jpg";

struct Handler {
    vao: VertexArray,
    texture: Texture2D,
    resolution: (f32, f32),
    program: Program
}

impl Handler {
    fn new(win: &GLWindow) -> Result<Handler, String> {
    	let resolution = (win.width() as f32, win.height() as f32);
        let vertices: [f32; 24] = [
             // positions // texture coords
             1.0,  1.0,   1.0, 1.0, // top right
             1.0, -1.0,   1.0, 0.0, // bottom right
            -1.0,  1.0,   0.0, 1.0, // top left
             1.0, -1.0,   1.0, 0.0, // bottom right
            -1.0, -1.0,   0.0, 0.0, // bottom left
            -1.0,  1.0,   0.0, 1.0  // top left
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
        
        texture.parameter_2d(gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        texture.parameter_2d(gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        texture.parameter_2d(gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        texture.parameter_2d(gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let mut img = PixelArray::load_jpg_from_path(IMG_PATH).unwrap();
        img.flipv();
        texture.set_image_2d(img);
        texture.generate_mipmap();
        Ok(Handler { vao, texture, resolution, program })
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self) -> Result<(), String> {
        // All drawing code should be put here
        unsafe {
            let res_uniform = Uniform::new(&self.program, "u_resolution")?;
            res_uniform.uniform2f(self.resolution.0, self.resolution.1);
                        
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

    let (app, window) = GLWindow::new_with_title("Black hole raytracer")?;
    window.get_context()?;
    gl_info();
    
    let render_handler = Handler::new(&window)?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
