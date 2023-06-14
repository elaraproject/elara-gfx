// This demo shows how to draw polygons and other shapes
// using elara-gfx
use elara_gfx::{gl_info, Buffer, BufferType, Program, Shader, VertexArray};
use elara_gfx::{GLWindow, HandlerResult, WindowHandler};
use elara_log::prelude::*;
use std::error::Error;
use std::f32::consts::PI;


const VERT_SHADER: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER: &str = include_str!("shaders/triangle.frag");

struct Handler {
    vao: VertexArray,
    vertex_num: usize
}

#[derive(Clone, Debug)]
struct Vertex(f32, f32);

// struct Color(f32, f32, f32, f32);

#[derive(Debug)]
struct Canvas {
    points: Vec<Vec<[f32; 3]>>
//    colors: Vec<Color>
}

impl Canvas {
    fn new() -> Canvas {
        Canvas { points: Vec::new() }
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

    // Creates a rectangle with top-left corner
    // at (x, y) with a width of w and a height of h
    // internally creates 2 triangles
    fn add_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let p1 = [x + w, y, 0.0]; // top right
        let p2 = [x + w, y + h, 0.0]; // bottom right
        let p3 = [x, y, 0.0]; // top left
        let p4 = [x + w, y + h, 0.0]; // bottom right
        let p5 = [x, y + h, 0.0]; // bottom left
        let p6 = [x, y, 0.0]; // top left
        let rect = vec![p1, p2, p3, p4, p5, p6];
        self.add_shape(rect);
    }

    // Creates a polygon with center at (x, y)
    // and radius of r; internally creates a polygon 
    // composed of triangles
    fn add_polygon(&mut self, x: f32, y: f32, r: f32, sides: i32) {
        let mut theta = (2.0 * PI) / (sides as f32);
        let mut polygon = Vec::new();
        for i in 0..sides {
            let p1 = [x + r * (i as f32 * theta).cos(), y + r * (i as f32 * theta).sin(), 0.0];
            let p2 = [x + r * ((i - 1) as f32 * theta).cos(), y + r * ((i - 1) as f32 * theta).sin(), 0.0];
            let p3 = [x, y, 0.0];
            polygon.push(p1);
            polygon.push(p2);
            polygon.push(p3);
        }
        self.add_shape(polygon);
    }
    
    // Creates a circle with center at (x, y)
    // and a radius of r; essentially a very
    // well-subdivided polygon
    fn add_circle(&mut self, x: f32, y: f32, r: f32) {
        const CIRCLE_SUBDIVISIONS: i32 = 128;
        self.add_polygon(x, y, r, CIRCLE_SUBDIVISIONS);
    }

    fn add_shape(&mut self, vertex: Vec<[f32; 3]>) {
        self.points.push(vertex);
    }
    
    fn to_vertices(&self) -> Vec<f32> {
        let mut vertices = Vec::new();
        for shape in self.points.iter() {
            for vertex in shape.iter() {
                vertices.push(vertex[0]);
                vertices.push(vertex[1]);
                vertices.push(vertex[2]);
            }
        }
        vertices
    }
}

impl Handler {
    fn new() -> Result<Handler, String> {
        // Draw code here
        let mut canvas = Canvas::new();
        // canvas.add_rect(-0.5, 0.0, 0.8, 0.5);
        // canvas.add_polygon(0.0, 0.0, 0.3, 6);
        // canvas.add_rect(0.1, 0.5, 0.4, 0.3);
        canvas.add_circle(0.0, 0.0, 0.4);
        let vertices = &canvas.to_vertices();
        let vertex_num = canvas.len();
        println!("{:?}", vertices);
        let vao = VertexArray::new()?;
        vao.bind();

        let vbo = Buffer::new()?;
        vbo.bind(BufferType::Array);
        vbo.data::<f32>(BufferType::Array, &vertices, gl::STATIC_DRAW);
        // vao.vertex_attrib_pointer(0, vertices.len() as i32, gl::FLOAT, false, 0);
        vao.vertex_attrib_pointer(0, 3, gl::FLOAT, false, 0);
        vao.enable_vertex_attrib(0);

        vao.unbind();
        vbo.unbind(BufferType::Array);

        let vertex_shader = Shader::new(&VERT_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&FRAG_SHADER, gl::FRAGMENT_SHADER)?;
        let program = Program::new(&[vertex_shader, fragment_shader])?;
        program.use_program();

        Ok(Handler {
            vao,
            vertex_num
        })
    }
}


impl WindowHandler for Handler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            // Temporary fix so circles are not stretched
            // gl::Viewport(0, 0, 600, 600);
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
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
