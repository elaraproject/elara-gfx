use std::f32::consts::PI;
use crate::GLWindow;

pub const ATLAS_IMG: &[u8] = include_bytes!("resources/font-tex.png");
pub const ATLAS_WIDTH: f32 = 358.0;
pub const ATLAS_HEIGHT: f32 = 133.0;
pub const ATLAS_CHARS: [char; 95] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ', '!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~'
];
pub const ATLAS: [CharCoord; 95] = [
    CharCoord {  x: 65, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 62, y: 88, w: 12, h: 26, originX: -1, originY: 25, advance: 18 },
    CharCoord {  x: 274, y: 62, w: 18, h: 26, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 83, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 155, y: 62, w: 20, h: 26, originX: 1, originY: 25, advance: 18 },
    CharCoord {  x: 101, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 119, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 292, y: 62, w: 18, h: 26, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 137, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 155, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 199, y: 114, w: 3, h: 3, originX: 1, originY: 1, advance: 8 },
    CharCoord {  x: 257, y: 35, w: 7, h: 27, originX: -1, originY: 25, advance: 8 },
    CharCoord {  x: 91, y: 114, w: 12, h: 12, originX: 0, originY: 25, advance: 13 },
    CharCoord {  x: 0, y: 62, w: 23, h: 26, originX: 1, originY: 25, advance: 21 },
    CharCoord {  x: 151, y: 0, w: 18, h: 29, originX: 0, originY: 26, advance: 18 },
    CharCoord {  x: 281, y: 0, w: 27, h: 27, originX: 0, originY: 25, advance: 26 },
    CharCoord {  x: 308, y: 0, w: 25, h: 27, originX: 0, originY: 25, advance: 23 },
    CharCoord {  x: 111, y: 114, w: 7, h: 12, originX: 0, originY: 25, advance: 7 },
    CharCoord {  x: 102, y: 0, w: 10, h: 32, originX: 0, originY: 25, advance: 9 },
    CharCoord {  x: 69, y: 0, w: 11, h: 32, originX: 1, originY: 25, advance: 9 },
    CharCoord {  x: 73, y: 114, w: 18, h: 17, originX: 0, originY: 26, advance: 18 },
    CharCoord {  x: 0, y: 114, w: 18, h: 19, originX: 0, originY: 21, advance: 18 },
    CharCoord {  x: 103, y: 114, w: 8, h: 12, originX: 0, originY: 6, advance: 8 },
    CharCoord {  x: 170, y: 114, w: 11, h: 6, originX: 0, originY: 12, advance: 10 },
    CharCoord {  x: 163, y: 114, w: 7, h: 7, originX: -1, originY: 5, advance: 8 },
    CharCoord {  x: 34, y: 88, w: 14, h: 26, originX: 1, originY: 25, advance: 12 },
    CharCoord {  x: 211, y: 88, w: 7, h: 21, originX: -1, originY: 19, advance: 8 },
    CharCoord {  x: 99, y: 88, w: 9, h: 25, originX: 1, originY: 19, advance: 8 },
    CharCoord {  x: 18, y: 114, w: 18, h: 19, originX: 0, originY: 21, advance: 18 },
    CharCoord {  x: 118, y: 114, w: 18, h: 11, originX: 0, originY: 17, advance: 18 },
    CharCoord {  x: 36, y: 114, w: 18, h: 19, originX: 0, originY: 21, advance: 18 },
    CharCoord {  x: 226, y: 35, w: 16, h: 27, originX: 1, originY: 25, advance: 14 },
    CharCoord {  x: 122, y: 0, w: 29, h: 29, originX: 0, originY: 25, advance: 29 },
    CharCoord {  x: 328, y: 35, w: 24, h: 26, originX: 2, originY: 25, advance: 20 },
    CharCoord {  x: 175, y: 62, w: 20, h: 26, originX: -1, originY: 25, advance: 21 },
    CharCoord {  x: 23, y: 35, w: 21, h: 27, originX: 0, originY: 25, advance: 20 },
    CharCoord {  x: 46, y: 62, w: 22, h: 26, originX: -1, originY: 25, advance: 23 },
    CharCoord {  x: 328, y: 62, w: 17, h: 26, originX: -1, originY: 25, advance: 18 },
    CharCoord {  x: 0, y: 88, w: 17, h: 26, originX: -1, originY: 25, advance: 17 },
    CharCoord {  x: 0, y: 35, w: 23, h: 27, originX: 0, originY: 25, advance: 23 },
    CharCoord {  x: 134, y: 62, w: 21, h: 26, originX: -1, originY: 25, advance: 24 },
    CharCoord {  x: 74, y: 88, w: 6, h: 26, originX: -1, originY: 25, advance: 9 },
    CharCoord {  x: 80, y: 0, w: 11, h: 32, originX: 4, originY: 25, advance: 9 },
    CharCoord {  x: 195, y: 62, w: 20, h: 26, originX: -1, originY: 25, advance: 20 },
    CharCoord {  x: 17, y: 88, w: 17, h: 26, originX: -1, originY: 25, advance: 17 },
    CharCoord {  x: 302, y: 35, w: 26, h: 26, originX: -1, originY: 25, advance: 29 },
    CharCoord {  x: 68, y: 62, w: 22, h: 26, originX: -1, originY: 25, advance: 24 },
    CharCoord {  x: 333, y: 0, w: 25, h: 27, originX: 0, originY: 25, advance: 25 },
    CharCoord {  x: 310, y: 62, w: 18, h: 26, originX: -1, originY: 25, advance: 19 },
    CharCoord {  x: 16, y: 0, w: 25, h: 32, originX: 0, originY: 25, advance: 25 },
    CharCoord {  x: 215, y: 62, w: 20, h: 26, originX: -1, originY: 25, advance: 20 },
    CharCoord {  x: 173, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 235, y: 62, w: 20, h: 26, originX: 1, originY: 25, advance: 18 },
    CharCoord {  x: 44, y: 35, w: 21, h: 27, originX: -1, originY: 25, advance: 23 },
    CharCoord {  x: 23, y: 62, w: 23, h: 26, originX: 2, originY: 25, advance: 19 },
    CharCoord {  x: 270, y: 35, w: 32, h: 26, originX: 1, originY: 25, advance: 30 },
    CharCoord {  x: 90, y: 62, w: 22, h: 26, originX: 2, originY: 25, advance: 19 },
    CharCoord {  x: 112, y: 62, w: 22, h: 26, originX: 2, originY: 25, advance: 18 },
    CharCoord {  x: 255, y: 62, w: 19, h: 26, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 112, y: 0, w: 10, h: 32, originX: -1, originY: 25, advance: 10 },
    CharCoord {  x: 48, y: 88, w: 14, h: 26, originX: 1, originY: 25, advance: 12 },
    CharCoord {  x: 91, y: 0, w: 11, h: 32, originX: 1, originY: 25, advance: 10 },
    CharCoord {  x: 54, y: 114, w: 19, h: 18, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 181, y: 114, w: 18, h: 5, originX: 2, originY: -1, advance: 14 },
    CharCoord {  x: 136, y: 114, w: 9, h: 8, originX: 0, originY: 26, advance: 9 },
    CharCoord {  x: 163, y: 88, w: 17, h: 21, originX: 0, originY: 19, advance: 18 },
    CharCoord {  x: 227, y: 0, w: 18, h: 28, originX: -1, originY: 26, advance: 20 },
    CharCoord {  x: 180, y: 88, w: 16, h: 21, originX: 0, originY: 19, advance: 15 },
    CharCoord {  x: 189, y: 0, w: 19, h: 28, originX: 0, originY: 26, advance: 20 },
    CharCoord {  x: 127, y: 88, w: 18, h: 21, originX: 0, originY: 19, advance: 18 },
    CharCoord {  x: 242, y: 35, w: 15, h: 27, originX: 1, originY: 26, advance: 11 },
    CharCoord {  x: 208, y: 0, w: 19, h: 28, originX: 1, originY: 19, advance: 17 },
    CharCoord {  x: 191, y: 35, w: 18, h: 27, originX: -1, originY: 26, advance: 20 },
    CharCoord {  x: 80, y: 88, w: 6, h: 26, originX: -1, originY: 25, advance: 8 },
    CharCoord {  x: 6, y: 0, w: 10, h: 34, originX: 3, originY: 25, advance: 8 },
    CharCoord {  x: 209, y: 35, w: 17, h: 27, originX: -1, originY: 26, advance: 17 },
    CharCoord {  x: 264, y: 35, w: 6, h: 27, originX: -1, originY: 26, advance: 8 },
    CharCoord {  x: 218, y: 88, w: 28, h: 20, originX: -1, originY: 19, advance: 30 },
    CharCoord {  x: 312, y: 88, w: 18, h: 20, originX: -1, originY: 19, advance: 20 },
    CharCoord {  x: 108, y: 88, w: 19, h: 21, originX: 0, originY: 19, advance: 19 },
    CharCoord {  x: 245, y: 0, w: 18, h: 28, originX: -1, originY: 19, advance: 20 },
    CharCoord {  x: 263, y: 0, w: 18, h: 28, originX: 0, originY: 19, advance: 20 },
    CharCoord {  x: 345, y: 88, w: 13, h: 20, originX: -1, originY: 19, advance: 13 },
    CharCoord {  x: 196, y: 88, w: 15, h: 21, originX: 0, originY: 19, advance: 15 },
    CharCoord {  x: 86, y: 88, w: 13, h: 25, originX: 1, originY: 23, advance: 11 },
    CharCoord {  x: 145, y: 88, w: 18, h: 21, originX: -1, originY: 19, advance: 20 },
    CharCoord {  x: 273, y: 88, w: 20, h: 20, originX: 2, originY: 19, advance: 16 },
    CharCoord {  x: 246, y: 88, w: 27, h: 20, originX: 1, originY: 19, advance: 25 },
    CharCoord {  x: 293, y: 88, w: 19, h: 20, originX: 1, originY: 19, advance: 17 },
    CharCoord {  x: 169, y: 0, w: 20, h: 28, originX: 2, originY: 19, advance: 16 },
    CharCoord {  x: 330, y: 88, w: 15, h: 20, originX: 0, originY: 19, advance: 15 },
    CharCoord {  x: 41, y: 0, w: 14, h: 32, originX: 1, originY: 25, advance: 12 },
    CharCoord {  x: 0, y: 0, w: 6, h: 35, originX: -6, originY: 26, advance: 18 },
    CharCoord {  x: 55, y: 0, w: 14, h: 32, originX: 1, originY: 25, advance: 12 },
    CharCoord {  x: 145, y: 114, w: 18, h: 7, originX: 0, originY: 15, advance: 18 },
];

pub fn get_charcoord_from_char(character: char) -> Option<CharCoord> {
    let index = ATLAS_CHARS.iter().position(|&c| c == character);
    if let Some(idx) = index {
        Some(ATLAS[idx])
    } else {
        None
    }
}

pub fn subtract_vertices(x: [f32; 2], y: [f32; 2]) -> [f32; 2] {
    let x_out = x[0] - y[0];
    let y_out = x[1] - y[1];
    [x_out, y_out]
}


pub fn add_vertices(x: [f32; 2], y: [f32; 2]) -> [f32; 2] {
    let x_out = y[0] + x[0];
    let y_out = y[1] + x[1];
    [x_out, y_out]
}

pub fn vector_norm(x: [f32; 2]) -> f32 {
    let norm: f32 = x.into_iter().map(|x| x * x).sum::<f32>().sqrt();
    norm
} 

pub fn abs_normalize_2d(x: [f32; 2], norm: f32, scale: f32) -> [f32; 2] {
    let x_out = x[0].abs() / (scale * norm);
    let y_out = x[1].abs() / (scale * norm);
    [x_out, y_out]
}

#[derive(Clone, Debug)]
pub struct Vertex(f32, f32);

#[derive(Clone, Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

#[derive(Debug)]
pub struct Canvas {
    points: Vec<Vec<[f32; 8]>>,
    background: Color,
    aspect_ratio: f32
}

pub struct TexCoord;

impl TexCoord {
    pub fn default() -> [f32; 8] {
        [0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0] 
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CharCoord {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub originX: i32,
    pub originY: i32,
    pub advance: i32
}

impl Canvas {
    pub fn new(win: &GLWindow) -> Canvas {
        Canvas { 
            points: Vec::new(), 
            background: Color(1.0, 1.0, 1.0, 1.0),
            aspect_ratio: win.height() as f32 / win.width() as f32 
        }
    }
    
    pub fn len(&self) -> usize {
        let mut len = 0_usize;
        for shape in self.points.iter() {
            for _vertex in shape.iter() {
                len += 1_usize;
            }
        }
        len
    }
    
    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }
    
    pub fn background(self) -> Color {
        self.background
    }

    // Creates a rectangle with top-left corner
    // at (x, y) with a width of w and a height of h
    // internally creates 2 triangles
    pub fn add_rect(&mut self, x: f32, y: f32, w: f32, h: f32, fill: Color) {
        let p1 = [x, y, fill.0, fill.1, fill.2, fill.3, 1.0, 1.0]; // top left
        let p2 = [x + w, y, fill.0, fill.1, fill.2, fill.3, 1.0, 1.0]; // top right
        let p3 = [x + w, y + h, fill.0, fill.1, fill.2, fill.3, 1.0, 1.0]; // bottom right
        let p4 = [x, y + h, fill.0, fill.1, fill.2, fill.3, 1.0, 1.0]; // bottom left
        let rect = vec![p2, p3, p1, p3, p4, p1];
        self.add_shape(rect);
    }

    // Creates a quad with 4 vertices going clockwise
    // from the top-left
    pub fn add_quad(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], fill: Color) {
        let point1 = [p1[0], p1[1], fill.0, fill.1, fill.2, fill.3, 1.0, 1.0]; // top left
        let point2 = [p2[0], p2[1], fill.0, fill.1, fill.2, fill.3, 1.0, 1.0]; // top right
        let point3 = [p3[0], p3[1], fill.0, fill.1, fill.2, fill.3, 1.0, 1.0]; // bottom right
        let point4 = [p4[0], p4[1], fill.0, fill.1, fill.2, fill.3, 1.0, 1.0]; // bottom left
        let quad = vec![point2, point3, point1, point3, point4, point1];
        self.add_shape(quad);
    }

    // Creates a triangle with 3 vertices going clockwise
    // from the top
    pub fn add_triangle(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], fill: Color) {
        let point1 = [p1[0], p1[1], fill.0, fill.1, fill.2, fill.3, 1.0, 1.0];
        let point2 = [p2[0], p2[1], fill.0, fill.1, fill.2, fill.3, 1.0, 1.0];
        let point3 = [p3[0], p3[1], fill.0, fill.1, fill.2, fill.3, 1.0, 1.0];
        let triangle = vec![point1, point2, point3];
        self.add_shape(triangle);
    }
    
    // Creates a square with top-left corner
    // at (x, y) with a width of w
    pub fn add_square(&mut self, x: f32, y: f32, w: f32, fill: Color) {
        self.add_rect(x, y, w, w, fill);
    }

    // Creates a polygon with center at (x, y)
    // and radius of r; internally creates a polygon 
    // composed of triangles
    pub fn add_polygon(&mut self, x: f32, y: f32, r: f32, sides: i32, fill: Color) {
        let theta = (2.0 * PI) / (sides as f32);
        let mut polygon = Vec::new();
        for i in 0..sides {
            let p1 = [x + r * (i as f32 * theta).cos(), y + r * (i as f32 * theta).sin(), fill.0, fill.1, fill.2, fill.3, 1.0, 1.0];
            let p2 = [x + r * ((i - 1) as f32 * theta).cos(), y + r * ((i - 1) as f32 * theta).sin(), fill.0, fill.1, fill.2, fill.3, 1.0, 1.0];
            let p3 = [x, y, fill.0, fill.1, fill.2, fill.3, 1.0, 1.0];
            polygon.push(p1);
            polygon.push(p2);
            polygon.push(p3);
        }
        self.add_shape(polygon);
    }
    
    // Creates a circle with center at (x, y)
    // and a radius of r; essentially a very
    // well-subdivided polygon
    pub fn add_circle(&mut self, x: f32, y: f32, r: f32, fill: Color) {
        const CIRCLE_SUBDIVISIONS: i32 = 128;
        self.add_polygon(x, y, r, CIRCLE_SUBDIVISIONS, fill);
    }

    // Creates a line from a path of points
    // with paramters line width `width`, and optionally whether
    // to form a closed loop `close_loop`
    pub fn add_line(&mut self, path: Vec<[f32; 2]>, width: f32, fill: Color, close_loop: bool) {
        // let mut line = Vec::new();
        let mut edge_normals: Vec<[f32; 2]> = Vec::	new();
        let mut vertex_normals = Vec::new();
        let n = path.len();
        for i in 0..n {
            let j = (i + 1) % n;
            let edge_tangent = subtract_vertices(path[j], path[i]);
            let edge_normal = [-edge_tangent[1], edge_tangent[0]];
            edge_normals.push(edge_normal);
        }
        for i in 0..n {
            let j = (n + i - 1) % n;
            let vertex_normal = add_vertices(edge_normals[i], edge_normals[j]);
            let norm = vector_norm(vertex_normal);
            let normalized_vector_normal = abs_normalize_2d(vertex_normal, norm, 500.0 / (width));
            vertex_normals.push(normalized_vector_normal);
        }
        let mut num_iters = n - 1;
        if close_loop == true {
            num_iters = n;
        }
        for i in 0..num_iters {
            let j = (i + 1) % n;
            let p1 = add_vertices(path[i], vertex_normals[i]);
            let p2 = add_vertices(path[j], vertex_normals[j]);
            let p3 = subtract_vertices(path[j], vertex_normals[j]);
            let p4 = subtract_vertices(path[i], vertex_normals[i]);
            self.add_quad(p1, p2, p3, p4, fill.clone());
        }
    }

    pub fn add_shape(&mut self, vertex: Vec<[f32; 8]>) {
        self.points.push(vertex);
    }

    // Adds a textured quad for loading an image, with given 
    // texture coordinates starting from the top left going clockwise
    pub fn add_image(&mut self, x: f32, y: f32, w: f32, h: f32, texcoord: [f32; 8]) {
        let p1 = [x, y, 1.0, 1.0, 1.0, 0.0, texcoord[0], texcoord[1]];
        let p2 = [x + w, y, 1.0, 1.0, 1.0, 0.0, texcoord[2], texcoord[3]];
        let p3 = [x + w, y + h, 1.0, 1.0, 1.0, 0.0, texcoord[4], texcoord[5]];
        let p4 = [x, y + h, 1.0, 1.0, 1.0, 0.0, texcoord[6], texcoord[7]];
        let image = vec![p2, p3, p1, p3, p4, p1];
        self.add_shape(image);
    }

    pub fn add_text(&mut self, x0: f32, y0: f32, text: &str, size: f32) {
        let mut x = x0;
        let y = y0;
        // let w = size * 0.005;
        for char in text.chars() {
            let character_tex = get_charcoord_from_char(char).unwrap();
            // Top left (x, y+h)
            let s0 = character_tex.x as f32 / ATLAS_WIDTH;
            let t0 = (character_tex.y + character_tex.h) as f32 / ATLAS_HEIGHT;
            // Top right (x+h, y+h)
            let s1 = (character_tex.x + character_tex.w) as f32 / ATLAS_WIDTH;
            let t1 = (character_tex.y + character_tex.h) as f32 / ATLAS_HEIGHT;
            // Bottom right (x+h, y)
            let s2 = (character_tex.x + character_tex.w) as f32 / ATLAS_WIDTH;
            let t2 = character_tex.y as f32 / ATLAS_HEIGHT;
            // Bottom left (x, y)
            let s3 = character_tex.x as f32 / ATLAS_WIDTH;
            let t3 = character_tex.y as f32 / ATLAS_HEIGHT;
            // Texcoords
            let texcoords = [s0, t0, s1, t1, s2, t2, s3, t3];
            let w = (character_tex.w as f32) / (ATLAS_WIDTH * size * 0.1 * self.aspect_ratio);
            let h = character_tex.h as f32 / (ATLAS_HEIGHT * size * 0.1);
            self.add_image(x, y, w, h, texcoords);
            x += (character_tex.advance as f32) / (ATLAS_WIDTH * size * 0.1 * self.aspect_ratio);
        }
    }

    pub fn to_vertices(&self) -> Vec<f32> {
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
