use std::collections::HashMap;
use std::f32::consts::PI;
use crate::{GLWindow, VertexArray, Texture2D, Program, Draw, PixelArray, WindowHandler, Buffer, BufferType, Shader, HandlerResult, Uniform};
use crate::types;
use std::ffi::OsStr;
use freetype::Library;
use freetype::face::LoadFlag;

const TEXT_VERTEX_SHADER: &'static str = r#"
#version 330 core
in vec4 vertex;
out vec2 TexCoord;

void main() {
    gl_Position = vec4(vertex.xy, 0.0, 1.0);
    TexCoord = vertex.zw;
}
"#;

const TEXT_FRAGMENT_SHADER: &'static str = r#"
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


pub const ATLAS_IMG_BLACK: &[u8] = include_bytes!("resources/font-tex.png");
pub const ATLAS_IMG_WHITE: &[u8] = include_bytes!("resources/font-tex-white.png");
pub const ATLAS_WIDTH_BLACK: f32 = 358.0;
pub const ATLAS_HEIGHT_BLACK: f32 = 133.0;
pub const ATLAS_WIDTH_WHITE: f32 = 353.0;
pub const ATLAS_HEIGHT_WHITE: f32 = 134.0;
pub const ATLAS_FONT_SIZE: i32 = 32;
pub const ATLAS_CHARS: [char; 95] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ', '!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~'
];
pub const ATLAS_WHITE: [CharCoord; 95] = [
    CharCoord {  x: 90, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 79, y: 88, w: 12, h: 26, originX: -1, originY: 25, advance: 18 },
    CharCoord {  x: 299, y: 62, w: 18, h: 26, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 108, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 179, y: 62, w: 20, h: 26, originX: 1, originY: 25, advance: 18 },
    CharCoord {  x: 126, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 144, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 317, y: 62, w: 18, h: 26, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 162, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 180, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 228, y: 114, w: 3, h: 3, originX: 1, originY: 1, advance: 8 },
    CharCoord {  x: 267, y: 35, w: 7, h: 27, originX: -1, originY: 25, advance: 8 },
    CharCoord {  x: 138, y: 114, w: 12, h: 12, originX: 0, originY: 25, advance: 13 },
    CharCoord {  x: 24, y: 62, w: 23, h: 26, originX: 1, originY: 25, advance: 21 },
    CharCoord {  x: 171, y: 0, w: 18, h: 29, originX: 0, originY: 26, advance: 18 },
    CharCoord {  x: 300, y: 0, w: 27, h: 27, originX: 0, originY: 25, advance: 26 },
    CharCoord {  x: 327, y: 0, w: 25, h: 27, originX: 0, originY: 25, advance: 23 },
    CharCoord {  x: 158, y: 114, w: 7, h: 12, originX: 0, originY: 25, advance: 7 },
    CharCoord {  x: 75, y: 0, w: 10, h: 32, originX: 0, originY: 25, advance: 9 },
    CharCoord {  x: 42, y: 0, w: 11, h: 32, originX: 1, originY: 25, advance: 9 },
    CharCoord {  x: 102, y: 114, w: 18, h: 17, originX: 0, originY: 26, advance: 18 },
    CharCoord {  x: 29, y: 114, w: 18, h: 19, originX: 0, originY: 21, advance: 18 },
    CharCoord {  x: 150, y: 114, w: 8, h: 12, originX: 0, originY: 6, advance: 8 },
    CharCoord {  x: 199, y: 114, w: 11, h: 6, originX: 0, originY: 12, advance: 10 },
    CharCoord {  x: 174, y: 114, w: 7, h: 8, originX: -1, originY: 6, advance: 8 },
    CharCoord {  x: 51, y: 88, w: 14, h: 26, originX: 1, originY: 25, advance: 12 },
    CharCoord {  x: 126, y: 88, w: 7, h: 22, originX: -1, originY: 20, advance: 8 },
    CharCoord {  x: 91, y: 88, w: 9, h: 26, originX: 1, originY: 20, advance: 8 },
    CharCoord {  x: 47, y: 114, w: 18, h: 19, originX: 0, originY: 21, advance: 18 },
    CharCoord {  x: 120, y: 114, w: 18, h: 12, originX: 0, originY: 18, advance: 18 },
    CharCoord {  x: 65, y: 114, w: 18, h: 19, originX: 0, originY: 21, advance: 18 },
    CharCoord {  x: 251, y: 35, w: 16, h: 27, originX: 1, originY: 25, advance: 14 },
    CharCoord {  x: 123, y: 0, w: 29, h: 29, originX: 0, originY: 25, advance: 29 },
    CharCoord {  x: 0, y: 62, w: 24, h: 26, originX: 2, originY: 25, advance: 20 },
    CharCoord {  x: 199, y: 62, w: 20, h: 26, originX: -1, originY: 25, advance: 21 },
    CharCoord {  x: 48, y: 35, w: 21, h: 27, originX: 0, originY: 25, advance: 20 },
    CharCoord {  x: 70, y: 62, w: 22, h: 26, originX: -1, originY: 25, advance: 23 },
    CharCoord {  x: 0, y: 88, w: 17, h: 26, originX: -1, originY: 25, advance: 18 },
    CharCoord {  x: 17, y: 88, w: 17, h: 26, originX: -1, originY: 25, advance: 17 },
    CharCoord {  x: 25, y: 35, w: 23, h: 27, originX: 0, originY: 25, advance: 23 },
    CharCoord {  x: 158, y: 62, w: 21, h: 26, originX: -1, originY: 25, advance: 24 },
    CharCoord {  x: 100, y: 88, w: 7, h: 26, originX: -1, originY: 25, advance: 9 },
    CharCoord {  x: 53, y: 0, w: 11, h: 32, originX: 4, originY: 25, advance: 9 },
    CharCoord {  x: 219, y: 62, w: 20, h: 26, originX: -1, originY: 25, advance: 20 },
    CharCoord {  x: 34, y: 88, w: 17, h: 26, originX: -1, originY: 25, advance: 17 },
    CharCoord {  x: 312, y: 35, w: 26, h: 26, originX: -1, originY: 25, advance: 29 },
    CharCoord {  x: 92, y: 62, w: 22, h: 26, originX: -1, originY: 25, advance: 24 },
    CharCoord {  x: 0, y: 35, w: 25, h: 27, originX: 0, originY: 25, advance: 25 },
    CharCoord {  x: 335, y: 62, w: 18, h: 26, originX: -1, originY: 25, advance: 19 },
    CharCoord {  x: 17, y: 0, w: 25, h: 32, originX: 0, originY: 25, advance: 25 },
    CharCoord {  x: 239, y: 62, w: 20, h: 26, originX: -1, originY: 25, advance: 20 },
    CharCoord {  x: 198, y: 35, w: 18, h: 27, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 259, y: 62, w: 20, h: 26, originX: 1, originY: 25, advance: 18 },
    CharCoord {  x: 69, y: 35, w: 21, h: 27, originX: -1, originY: 25, advance: 23 },
    CharCoord {  x: 47, y: 62, w: 23, h: 26, originX: 2, originY: 25, advance: 19 },
    CharCoord {  x: 280, y: 35, w: 32, h: 26, originX: 1, originY: 25, advance: 30 },
    CharCoord {  x: 114, y: 62, w: 22, h: 26, originX: 2, originY: 25, advance: 19 },
    CharCoord {  x: 136, y: 62, w: 22, h: 26, originX: 2, originY: 25, advance: 18 },
    CharCoord {  x: 279, y: 62, w: 20, h: 26, originX: 1, originY: 25, advance: 18 },
    CharCoord {  x: 85, y: 0, w: 10, h: 32, originX: -1, originY: 25, advance: 10 },
    CharCoord {  x: 65, y: 88, w: 14, h: 26, originX: 1, originY: 25, advance: 12 },
    CharCoord {  x: 64, y: 0, w: 11, h: 32, originX: 1, originY: 25, advance: 10 },
    CharCoord {  x: 83, y: 114, w: 19, h: 18, originX: 0, originY: 25, advance: 18 },
    CharCoord {  x: 210, y: 114, w: 18, h: 5, originX: 2, originY: -1, advance: 14 },
    CharCoord {  x: 165, y: 114, w: 9, h: 9, originX: 0, originY: 27, advance: 9 },
    CharCoord {  x: 188, y: 88, w: 17, h: 21, originX: 0, originY: 19, advance: 18 },
    CharCoord {  x: 209, y: 0, w: 19, h: 28, originX: -1, originY: 26, advance: 20 },
    CharCoord {  x: 205, y: 88, w: 16, h: 21, originX: 0, originY: 19, advance: 15 },
    CharCoord {  x: 228, y: 0, w: 19, h: 28, originX: 0, originY: 26, advance: 20 },
    CharCoord {  x: 152, y: 88, w: 18, h: 21, originX: 0, originY: 19, advance: 18 },
    CharCoord {  x: 285, y: 0, w: 15, h: 28, originX: 1, originY: 27, advance: 11 },
    CharCoord {  x: 152, y: 0, w: 19, h: 29, originX: 1, originY: 20, advance: 17 },
    CharCoord {  x: 216, y: 35, w: 18, h: 27, originX: -1, originY: 26, advance: 20 },
    CharCoord {  x: 107, y: 88, w: 6, h: 26, originX: -1, originY: 25, advance: 8 },
    CharCoord {  x: 0, y: 0, w: 11, h: 35, originX: 4, originY: 26, advance: 8 },
    CharCoord {  x: 234, y: 35, w: 17, h: 27, originX: -1, originY: 26, advance: 17 },
    CharCoord {  x: 274, y: 35, w: 6, h: 27, originX: -1, originY: 26, advance: 8 },
    CharCoord {  x: 237, y: 88, w: 28, h: 20, originX: -1, originY: 19, advance: 30 },
    CharCoord {  x: 331, y: 88, w: 18, h: 20, originX: -1, originY: 19, advance: 20 },
    CharCoord {  x: 133, y: 88, w: 19, h: 21, originX: 0, originY: 19, advance: 19 },
    CharCoord {  x: 247, y: 0, w: 19, h: 28, originX: -1, originY: 19, advance: 20 },
    CharCoord {  x: 266, y: 0, w: 19, h: 28, originX: 0, originY: 19, advance: 20 },
    CharCoord {  x: 16, y: 114, w: 13, h: 20, originX: -1, originY: 19, advance: 13 },
    CharCoord {  x: 221, y: 88, w: 16, h: 21, originX: 0, originY: 19, advance: 15 },
    CharCoord {  x: 113, y: 88, w: 13, h: 25, originX: 1, originY: 23, advance: 11 },
    CharCoord {  x: 170, y: 88, w: 18, h: 21, originX: -1, originY: 19, advance: 20 },
    CharCoord {  x: 292, y: 88, w: 20, h: 20, originX: 2, originY: 19, advance: 16 },
    CharCoord {  x: 265, y: 88, w: 27, h: 20, originX: 1, originY: 19, advance: 25 },
    CharCoord {  x: 312, y: 88, w: 19, h: 20, originX: 1, originY: 19, advance: 17 },
    CharCoord {  x: 189, y: 0, w: 20, h: 28, originX: 2, originY: 19, advance: 16 },
    CharCoord {  x: 0, y: 114, w: 16, h: 20, originX: 1, originY: 19, advance: 15 },
    CharCoord {  x: 95, y: 0, w: 14, h: 31, originX: 1, originY: 25, advance: 12 },
    CharCoord {  x: 11, y: 0, w: 6, h: 35, originX: -6, originY: 26, advance: 18 },
    CharCoord {  x: 109, y: 0, w: 14, h: 31, originX: 1, originY: 25, advance: 12 },
    CharCoord {  x: 181, y: 114, w: 18, h: 7, originX: 0, originY: 15, advance: 18 },
];

pub const ATLAS_BLACK: [CharCoord; 95] = [
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

const CANVAS_VERT_SHADER: &'static str = r#"
#version 330 core
in vec2 position;
in vec2 tex_coord;
in vec4 vertex_color;
out vec2 TexCoord;
out vec4 VertexColor;
uniform float aspect_ratio;

void main() {
    VertexColor = vertex_color;
    TexCoord = tex_coord;
    gl_Position = vec4(position.x, position.y, 0.0, 1.0);
}
"#;

const CANVAS_FRAG_SHADER: &'static str = r#"
#version 330 core
in vec2 TexCoord;
in vec4 VertexColor;
uniform sampler2D uTexture;
out vec4 FragColor;

void main() {
    vec4 col = texture(uTexture, TexCoord);
    FragColor = mix(VertexColor.rgba, col, col.a);
}
"#;

pub fn clear_color(color: Color) {
    unsafe {
        gl::ClearColor(color.0 as f32 / 255.0, color.1 as f32 / 255.0, color.2 as f32 / 255.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn get_charcoord_from_char(character: char, white_text: bool) -> Option<CharCoord> {
    let index = ATLAS_CHARS.iter().position(|&c| c == character);
    let ATLAS = if white_text { ATLAS_WHITE } else { ATLAS_BLACK };
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
pub struct Color(pub i32, pub i32, pub i32, pub f32);

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
            background: Color(255, 255, 255, 1.0),
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
        let p1 = [x, y, fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0]; // top left
        let p2 = [x + w, y, fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0]; // top right
        let p3 = [x + w, y + h, fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0]; // bottom right
        let p4 = [x, y + h, fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0]; // bottom left
        let rect = vec![p2, p3, p1, p3, p4, p1];
        self.add_shape(rect);
    }

    // Creates a quad with 4 vertices going clockwise
    // from the top-left
    pub fn add_quad(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], fill: Color) {
        let point1 = [p1[0], p1[1], fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0]; // top left
        let point2 = [p2[0], p2[1], fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0]; // top right
        let point3 = [p3[0], p3[1], fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0]; // bottom right
        let point4 = [p4[0], p4[1], fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0]; // bottom left
        let quad = vec![point2, point3, point1, point3, point4, point1];
        self.add_shape(quad);
    }

    // Creates a triangle with 3 vertices going clockwise
    // from the top
    pub fn add_triangle(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], fill: Color) {
        let point1 = [p1[0], p1[1], fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
        let point2 = [p2[0], p2[1], fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
        let point3 = [p3[0], p3[1], fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
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
            let p1 = [x + r * (i as f32 * theta).cos(), y + r * (i as f32 * theta).sin(), fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
            let p2 = [x + r * ((i - 1) as f32 * theta).cos(), y + r * ((i - 1) as f32 * theta).sin(), fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
            let p3 = [x, y, fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
            polygon.push(p1);
            polygon.push(p2);
            polygon.push(p3);
        }
        self.add_shape(polygon);
    }

    // Easter egg - draws a heart curve
    pub fn add_heart(&mut self, x: f32, y: f32, r: f32, fill: Color) {
        /*
            x(t) = r/19 * 16sin^3(t)
            y(t) = r/19 * (13cos(t) - 5cos(2t) -2cos(3t) - cos(4t)) 
         */
        self.add_parametric(x, y, |t| r / 19.0 * 16.0 * t.sin().powi(3), |t| r / 19.0 * (13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos()), 0.0, 2.0 * PI, fill)
    }

    // Draws a parametric curve from t = t0 to t = tf - for most situations use 0 to 2π
    pub fn add_parametric<F1, F2>(&mut self, x: f32, y: f32, x_t: F1, y_t: F2, t0: f32, tf: f32, fill: Color) 
    where F1: Fn(f32) -> f32, F2: Fn(f32) -> f32
    {
        let mut t = t0;
        let mut curve = Vec::new();
        let dt = 0.01_f32;
        while t < tf {
            let p1 = [x + x_t(t), y + y_t(t), fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
            let p2 = [x + x_t(t - dt), y + y_t(t - dt), fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
            let p3 = [x, y, fill.0 as f32 / 255.0, fill.1 as f32 / 255.0, fill.2 as f32 / 255.0, fill.3 as f32, 1.0, 1.0];
            curve.push(p1);
            curve.push(p2);
            curve.push(p3);
            t += dt;
        }
        self.add_shape(curve);
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

    pub fn add_text(&mut self, x0: f32, y0: f32, text: &str, size: f32, white_text: bool) {
        let ATLAS_WIDTH = if white_text { ATLAS_WIDTH_WHITE } else { ATLAS_WIDTH_BLACK };
        let ATLAS_HEIGHT = if white_text { ATLAS_HEIGHT_WHITE } else { ATLAS_HEIGHT_BLACK };
        let mut total_advance = 0;
        for char in text.chars() {
            let character_tex = get_charcoord_from_char(char, white_text).unwrap();
            total_advance += character_tex.advance;
        }
        let mut x = -(total_advance as f32 / 2.0);
        let y = (ATLAS_FONT_SIZE as f32 / 2.0);
        for char in text.chars() {
            // p0 --- p1
            // | \     |
            // |   \   |
            // |     \ |
            // p2 --- p3
            let character_tex = get_charcoord_from_char(char, white_text).unwrap();
            // Top left
            let x0 = (x - character_tex.originX as f32) / ATLAS_WIDTH; // p1
            let y0 = (y - character_tex.originY as f32) / ATLAS_HEIGHT;
            let s0 = character_tex.x as f32 / ATLAS_WIDTH;
            let t0 = character_tex.y as f32 / ATLAS_HEIGHT;
            // Top right
            let x1 = (x - character_tex.originX as f32 + character_tex.w as f32) / ATLAS_WIDTH; // p2
            let y1 = (y - character_tex.originY as f32) / ATLAS_HEIGHT;
            let s1 = (character_tex.x + character_tex.w) as f32 / ATLAS_WIDTH;
            let t1 = character_tex.y as f32 / ATLAS_HEIGHT;
            // Bottom left
            let x2 = (x - character_tex.originX as f32) / ATLAS_WIDTH; // p4
            let y2 = (y - character_tex.originY as f32 + character_tex.h as f32) / ATLAS_HEIGHT;
            let s2 = character_tex.x as f32 / ATLAS_WIDTH;
            let t2 = (character_tex.y + character_tex.h) as f32 / ATLAS_HEIGHT;
            // Bottom right
            let x3 = (x - character_tex.originX as f32 + character_tex.w as f32) / ATLAS_WIDTH; // p3
            let y3 = (y - character_tex.originY as f32 + character_tex.h as f32) / ATLAS_HEIGHT;
            let s3 = (character_tex.x + character_tex.w) as f32 / ATLAS_WIDTH;
            let t3 = (character_tex.y + character_tex.h) as f32 / ATLAS_HEIGHT;

            let p1 = [x0, y0, 1.0, 1.0, 1.0, 0.0, s0, t0];
            let p2 = [x1, y1, 1.0, 1.0, 1.0, 0.0, s1, t1];
            let p3 = [x3, y3, 1.0, 1.0, 1.0, 0.0, s3, t3];
            let p4 = [x0, y0, 1.0, 1.0, 1.0, 0.0, s0, t0];
            let p5 = [x3, y3, 1.0, 1.0, 1.0, 0.0, s3, t3];
            let p6 = [x2, y2, 1.0, 1.0, 1.0, 0.0, s2, t2];
            // Texcoords
            // let texcoords = [s0, t0, s1, t1, s2, t2, s3, t3];
            // let w = (character_tex.w as f32 * size * 0.03) / (ATLAS_WIDTH * self.aspect_ratio);
            // let h = (character_tex.h as f32 * size * 0.03) / ATLAS_HEIGHT;
            // let w = character_tex.w as f32 / ATLAS_WIDTH;
            // let h = character_tex.h as f32 / ATLAS_HEIGHT;
            // self.add_image(x - character_tex.originX as f32 / ATLAS_WIDTH, y / ATLAS_HEIGHT, w, h, texcoords);
            // x += (character_tex.advance as f32 * size * 0.03) / (ATLAS_WIDTH * self.aspect_ratio);
            // p1, p3, p0, p3, p4, p0
            x += character_tex.advance as f32;
            let text_quads = vec![p1, p2, p3, p4, p5, p6];
            self.add_shape(text_quads);
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

pub struct TextRenderer {
    program: Program,
    vao: VertexArray,
    vbo: Buffer,
    characters: HashMap<char, Character>
}

impl TextRenderer {
    pub fn new() -> Result<TextRenderer, String> {
        let vertex_shader = Shader::new(&TEXT_VERTEX_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&TEXT_FRAGMENT_SHADER, gl::FRAGMENT_SHADER)?;
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

        let characters: HashMap<char, Character> = HashMap::new();
        Ok(TextRenderer { program, vao, vbo, characters })
    }

    pub fn load<F: AsRef<OsStr>>(&mut self, font: F, size: u32) {
        if !self.characters.is_empty() {
            self.characters.clear();
        }

        let ft = Library::init().unwrap();
        let face = ft.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, size).unwrap();
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
                    face.glyph().bitmap().buffer().as_ptr() as *const types::c_void
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

                self.characters.insert(c as char, character);
            }
        }
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn render_text(&self, text: &str, x0: f32, y0: f32, scale: f32, color: Color) -> Result<(), String> {
        let mut x = x0;
        let y = y0;
        self.program.use_program();
        let color_uniform = Uniform::new(&self.program, "textColor")?;
        color_uniform.uniform3f(color.0 as f32 / 255.0, color.1 as f32 / 255.0, color.2 as f32 / 255.0);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
        }
        self.vao.bind();
        for c in text.chars() {
            let ch = self.characters.get(&c).unwrap();
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
                self.vbo.bind(BufferType::Array);
                self.vbo.subdata(BufferType::Array, 0, &vertices);
                self.vbo.unbind(BufferType::Array);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
                x += (ch.advance >> 6) as f32 * scale;
            }
        }
        self.vao.unbind();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Ok(())
    }
}

const RECT_VERTEX_SHADER: &'static str = r#"
#version 330 core
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const RECT_FRAGMENT_SHADER: &'static str = r#"
#version 330 core
uniform vec2 location; // location of the bottom left corner of rect
uniform vec2 size; // size of the rect in pixels
uniform vec3 rectColor; // rectangle color
uniform vec3 borderColor; // border color
uniform float borderThickness; // border thickness in pixels
uniform float borderRadius; // border radius in pixels

out vec4 fragColor;

// Based off https://www.shadertoy.com/view/fsdyzB
float roundedBoxSDF(vec2 CenterPosition, vec2 Size, float Radius) {
    vec2 q = abs(CenterPosition)-Size+Radius;
    return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - Radius;
}

void main() {
    float borderSoftness = 2.0;
    float edgeSoftness = 2.0;
    float distance = roundedBoxSDF(gl_FragCoord.xy - location - (size / 2.0), (size / 2.0), borderRadius);
    float smoothedAlpha = 1.0 - smoothstep(0.0, edgeSoftness, distance);
    float borderAlpha = 1.0 - smoothstep(borderThickness - borderSoftness, borderThickness, abs(distance));
    fragColor = vec4(mix(rectColor, borderColor, min(borderAlpha, smoothedAlpha)), smoothedAlpha);
}
"#;

pub struct RectStyle {
    x0: f32,
    y0: f32,
    w: f32,
    h: f32,
    rect_color: Color,
    border_color: Color,
    border_thickness: f32,
    border_radius: f32
}

impl RectStyle {
    pub fn new() -> RectStyle {
        RectStyle { x0: 50.0, y0: 50.0, w: 150.0, h: 50.0, rect_color: Color(255, 255, 255, 1.0), border_color: Color(0, 0, 0, 1.0), border_thickness: 0.0, border_radius: 0.0 }
    }

    pub fn position(mut self, x0: f32, y0: f32) -> Self {
        self.x0 = x0;
        self.y0 = y0;
        self
    }

    pub fn dims(mut self, w: u32, h: u32) -> Self {
        self.w = w as f32;
        self.h = h as f32;
        self
    }

    pub fn line_thickness(mut self, thickness: f32) -> Self {
        self.h = thickness;
        self
    }

    pub fn rect_color(mut self, r: i32, g: i32, b: i32) -> Self {
        self.rect_color = Color(r, g, b, 1.0);
        self
    }

    pub fn border_color(mut self, r: i32, g: i32, b: i32) -> Self {
        self.border_color = Color(r, g, b, 1.0);
        self
    }

    pub fn border_thickness(mut self, thickness: f32) -> Self {
        self.border_thickness = thickness;
        self
    }

    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }
}

pub struct RectRenderer {
    program: Program,
    vao: VertexArray,
    vbo: Buffer,
}

impl RectRenderer {
    pub fn new() -> Result<RectRenderer, String> {
        // Enable blending
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let vertex_shader = Shader::new(&RECT_VERTEX_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&RECT_FRAGMENT_SHADER, gl::FRAGMENT_SHADER)?;
        let program = Program::new(&[vertex_shader, fragment_shader])?;

        let vao = VertexArray::new()?;
        let vbo = Buffer::new()?;
        vao.bind();
        vbo.bind(BufferType::Array);
        vbo.data_empty::<f32>(BufferType::Array, 12, gl::DYNAMIC_DRAW);

        let vertex_attrib = vao.get_attrib_location(&program, "position");
        vao.enable_vertex_attrib(vertex_attrib as u32);
        vao.vertex_attrib_pointer::<f32>(vertex_attrib as u32, 2, gl::FLOAT, false, 2, 0);

        vbo.unbind(BufferType::Array);
        vao.unbind();

        Ok(RectRenderer { program, vao, vbo })
    }

    pub fn render_rect(&self, style: RectStyle) -> Result<(), String> {
        self.program.use_program();
        let location_uniform = Uniform::new(&self.program, "location")?;
        location_uniform.uniform2f(style.x0, style.y0);
        let size_uniform = Uniform::new(&self.program, "size")?;
        size_uniform.uniform2f(style.w, style.h);
        let color_uniform = Uniform::new(&self.program, "rectColor")?;
        color_uniform.uniform3f(style.rect_color.0 as f32 / 255.0, style.rect_color.1 as f32 / 255.0, style.rect_color.2 as f32 / 255.0);
        let border_color_uniform = Uniform::new(&self.program, "borderColor")?;
        border_color_uniform.uniform3f(style.border_color.0 as f32 / 255.0, style.border_color.1 as f32 / 255.0, style.border_color.2 as f32 / 255.0);
        let border_thickness_uniform = Uniform::new(&self.program, "borderThickness")?;
        border_thickness_uniform.uniform1f(style.border_thickness);
        let border_radius_uniform = Uniform::new(&self.program, "borderRadius")?;
        border_radius_uniform.uniform1f(style.border_radius);
        self.vao.bind();
        let vertices: [f32; 12] = [
            -1.0, 1.0,            
            -1.0, -1.0,    
            1.0, -1.0,    

            -1.0, 1.0,
            1.0, -1.0,    
            1.0, 1.0, 
        ];
        self.vbo.bind(BufferType::Array);
        self.vbo.subdata(BufferType::Array, 0, &vertices);
        self.vbo.unbind(BufferType::Array);
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
        Ok(())
    }
}

const LINE_VERTEX_SHADER: &'static str = r#"
#version 330 core
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const LINE_FRAGMENT_SHADER: &'static str = r#"
#version 330 core
uniform vec2 startLocation;
uniform vec2 endLocation;
uniform float thickness;
uniform vec3 lineColor;

out vec4 fragColor;

// source: https://computergraphics.stackexchange.com/questions/10682/what-is-the-most-efficient-line-algorithm-using-a-shader-program
float lineSegmentSDF(vec2 p, vec2 a, vec2 b)
{
   vec2 ba = b-a;
   vec2 pa = p-a;
   float h = clamp(dot(pa,ba)/dot(ba,ba), 0.0, 1.0);
   vec2 sqrd = pa-h*ba;
   sqrd = sqrd * sqrd;
   return sqrd.x+sqrd.y;
}

void main()
{
    // compute SDF
    float distance = lineSegmentSDF(gl_FragCoord.xy, startLocation, endLocation) - thickness;
    float edgeSoftness = 2.0;
    float smoothedAlpha = 1.0 - smoothstep(0.0, edgeSoftness, distance);
    fragColor = vec4(lineColor, smoothedAlpha);
}
"#;

pub struct LineRenderer {
    program: Program,
    vao: VertexArray,
    vbo: Buffer,
}

impl LineRenderer {
    pub fn new() -> Result<LineRenderer, String> {
        // Enable blending
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let vertex_shader = Shader::new(&LINE_VERTEX_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&LINE_FRAGMENT_SHADER, gl::FRAGMENT_SHADER)?;
        let program = Program::new(&[vertex_shader, fragment_shader])?;

        let vao = VertexArray::new()?;
        let vbo = Buffer::new()?;
        vao.bind();
        vbo.bind(BufferType::Array);
        vbo.data_empty::<f32>(BufferType::Array, 12, gl::DYNAMIC_DRAW);

        let vertex_attrib = vao.get_attrib_location(&program, "position");
        vao.enable_vertex_attrib(vertex_attrib as u32);
        vao.vertex_attrib_pointer::<f32>(vertex_attrib as u32, 2, gl::FLOAT, false, 2, 0);

        vbo.unbind(BufferType::Array);
        vao.unbind();
        Ok(LineRenderer{ program, vao, vbo })
    }

    // Render a line with start point p1 and end point p2;
    // it is recommended to use render_horizontal_line()
    // or render_vertical_line() instead
    pub fn render_line(&self, p1: [f32; 2], p2: [f32; 2], thickness: f32, color: Color) -> Result<(), String> {
        self.program.use_program();
        let start_location_uniform = Uniform::new(&self.program, "startLocation")?;
        start_location_uniform.uniform2f(p1[0], p1[1]);
        let end_location_uniform = Uniform::new(&self.program, "endLocation")?;
        end_location_uniform.uniform2f(p2[0], p2[1]);
        let thickness_uniform = Uniform::new(&self.program, "thickness")?;
        thickness_uniform.uniform1f(thickness);
        let color_uniform = Uniform::new(&self.program, "lineColor")?;
        color_uniform.uniform3f(color.0 as f32 / 255.0, color.1 as f32 / 255.0, color.2 as f32 / 255.0);

        self.vao.bind();
        let vertices: [f32; 12] = [
            -1.0, 1.0,            
            -1.0, -1.0,    
            1.0, -1.0,    

            -1.0, 1.0,
            1.0, -1.0,    
            1.0, 1.0, 
        ];
        self.vbo.bind(BufferType::Array);
        self.vbo.subdata(BufferType::Array, 0, &vertices);
        self.vbo.unbind(BufferType::Array);
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
        Ok(())
    }

    // Renders a vertical line with bottom at (x0, y0) and a height of h
    pub fn render_vertical_line(&self, x0: i32, y0: i32, h: i32, thickness: f32, color: Color) -> Result<(), String> {
        self.render_line([x0 as f32, y0 as f32], [x0 as f32, (y0 + h) as f32], thickness, color)
    }
    
    // Renders a horizontal line with left at (x0, y0) and a height of h
    pub fn render_horizontal_line(&self, x0: i32, y0: i32, w: i32, thickness: f32, color: Color) -> Result<(), String> {
        self.render_line([x0 as f32, y0 as f32], [(x0 + w) as f32, y0 as f32], thickness, color)
    }
}

pub struct CanvasHandler {
    vao: VertexArray,
    vertex_num: usize,
    background: Color,
    program: Program,
    aspect_ratio: f32,
    texture: Texture2D
}

impl CanvasHandler {
    pub fn new<D>(win: &GLWindow, mut canvas: D, white_text: bool) -> Result<CanvasHandler, String> 
        where D: Draw + 'static
    {
        let canvas = canvas.draw(&win)?;
        let ATLAS_IMG = if white_text { ATLAS_IMG_WHITE } else { ATLAS_IMG_BLACK };
        let img = PixelArray::load_png(ATLAS_IMG).unwrap();
        let vertices = &canvas.to_vertices();
        let vertex_num = canvas.len();
        let background = canvas.background();
        let aspect_ratio = win.height() as f32 / win.width() as f32;

        let texture = Texture2D::new()?;
        texture.bind();
        texture.parameter_2d(gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        texture.parameter_2d(gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        texture.parameter_2d(gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        texture.parameter_2d(gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        texture.enable_alpha_blend();

        texture.set_image_2d(img);
        texture.generate_mipmap();

        let vao = VertexArray::new()?;
        vao.bind();

        let vbo = Buffer::new()?;
        vbo.bind(BufferType::Array);
        vbo.data::<f32>(BufferType::Array, &vertices, gl::STATIC_DRAW);

        let vertex_shader = Shader::new(&CANVAS_VERT_SHADER, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(&CANVAS_FRAG_SHADER, gl::FRAGMENT_SHADER)?;
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

        Ok(CanvasHandler {
            vao,
            vertex_num,
            background,
            program,
            aspect_ratio,
            texture
        })
    }
}


impl WindowHandler for CanvasHandler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            let aspect_ratio_uniform = Uniform::new(&self.program, "aspect_ratio")?;
            aspect_ratio_uniform.uniform1f(self.aspect_ratio);
            gl::ClearColor(self.background.0 as f32 / 255.0, self.background.1 as f32 / 255.0, self.background.2 as f32 / 255.0, 1.0);
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
