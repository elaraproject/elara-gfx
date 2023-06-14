// This demo shows how to draw polygons and other shapes
// using elara-gfx
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, VertexArray};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;
use std::f32::consts::PI;


const VERT_SHADER: &str = include_str!("shaders/polygon.vert");
const FRAG_SHADER: &str = include_str!("shaders/polygon.frag");

struct Handler {
    vao: VertexArray,
    vertex_num: usize,
    background: Color
}

#[derive(Clone, Debug)]
struct Vertex(f32, f32);

#[derive(Clone, Debug)]
struct Color(f32, f32, f32);

#[derive(Debug)]
struct Canvas {
    points: Vec<Vec<[f32; 5]>>,
    background: Color
}

impl Canvas {
    fn new() -> Canvas {
        Canvas { points: Vec::new(), background: Color(1.0, 1.0, 1.0) }
    }
    
    fn len(&self) -> usize {
        let mut len = 0_usize;
        for shape in self.points.iter() {
            for vertex in shape.iter() {
                len += 1_usize;
            }
        }
        len
    }
    
    fn set_background(&mut self, color: Color) {
        self.background = color;
    }
    
    fn background(mut self) -> Color {
        self.background
    }

    // Creates a rectangle with top-left corner
    // at (x, y) with a width of w and a height of h
    // internally creates 2 triangles
    fn add_rect(&mut self, x: f32, y: f32, w: f32, h: f32, fill: Color) {
        let p1 = [x + w, y, fill.0, fill.1, fill.2]; // top right
        let p2 = [x + w, y + h, fill.0, fill.1, fill.2]; // bottom right
        let p3 = [x, y, fill.0, fill.1, fill.2]; // top left
        let p4 = [x + w, y + h, fill.0, fill.1, fill.2]; // bottom right
        let p5 = [x, y + h, fill.0, fill.1, fill.2]; // bottom left
        let p6 = [x, y, fill.0, fill.1, fill.2]; // top left
        let rect = vec![p1, p2, p3, p4, p5, p6];
        self.add_shape(rect);
    }
    
    // Creates a square with top-left corner
    // at (x, y) with a width of w
    fn add_square(&mut self, x: f32, y: f32, w: f32, fill: Color) {
        self.add_rect(x, y, w, w, fill);
    }

    // Creates a polygon with center at (x, y)
    // and radius of r; internally creates a polygon 
    // composed of triangles
    fn add_polygon(&mut self, x: f32, y: f32, r: f32, sides: i32, fill: Color) {
        let theta = (2.0 * PI) / (sides as f32);
        let mut polygon = Vec::new();
        for i in 0..sides {
            let p1 = [x + r * (i as f32 * theta).cos(), y + r * (i as f32 * theta).sin(), fill.0, fill.1, fill.2];
            let p2 = [x + r * ((i - 1) as f32 * theta).cos(), y + r * ((i - 1) as f32 * theta).sin(), fill.0, fill.1, fill.2];
            let p3 = [x, y, fill.0, fill.1, fill.2];
            polygon.push(p1);
            polygon.push(p2);
            polygon.push(p3);
        }
        self.add_shape(polygon);
    }
    
    // Creates a circle with center at (x, y)
    // and a radius of r; essentially a very
    // well-subdivided polygon
    fn add_circle(&mut self, x: f32, y: f32, r: f32, fill: Color) {
        const CIRCLE_SUBDIVISIONS: i32 = 128;
        self.add_polygon(x, y, r, CIRCLE_SUBDIVISIONS, fill);
    }

    fn add_shape(&mut self, vertex: Vec<[f32; 5]>) {
        self.points.push(vertex);
    }
    
    fn to_vertices(&self) -> Vec<f32> {
        let mut vertices = Vec::new();
        for shape in self.points.iter() {
            for vertex in shape.iter() {
                for point in vertex.iter() {
                    vertices.push(point.clone())
                }
            }
        }
        vertices
    }
}

impl Handler {
    fn new() -> Result<Handler, String> {
        // Draw code here
        let mut canvas = Canvas::new();
        canvas.set_background(Color(1.0, 1.0, 1.0));
        canvas.add_rect(-0.5, 0.0, 0.8, 0.5, Color(1.0, 0.0, 0.0));
        canvas.add_polygon(0.0, 0.0, 0.3, 6, Color(1.0, 0.0, 1.0));
        canvas.add_rect(0.1, 0.3, 0.4, 0.3, Color(0.0, 1.0, 0.0));
        canvas.add_circle(0.0, -0.2, 0.2, Color(0.0, 1.0, 1.0));
        
        // End draw code
        let vertices = &canvas.to_vertices();
        let vertex_num = canvas.len();
        let background = canvas.background();
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
        vao.vertex_attrib_pointer::<f32>(pos_attrib as u32, 2, gl::FLOAT, false, 5, 0);

        let color_attrib = vao.get_attrib_location(&program, "color");
        vao.enable_vertex_attrib(color_attrib as u32);
        vao.vertex_attrib_pointer::<f32>(color_attrib as u32, 3, gl::FLOAT, false, 5, 2);

        vao.unbind();
        vbo.unbind(BufferType::Array);

        Ok(Handler {
            vao,
            vertex_num,
            background
        })
    }
}


impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            // Temporary fix so circles are not stretched
            // gl::Viewport(0, 0, 900, 900);
            gl::ClearColor(self.background.0, self.background.1, self.background.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num as i32);
            self.vao.unbind();
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
    let render_handler = Handler::new()?;

    // Event handling
    app.run_loop(window, render_handler);
    Ok(())
}
