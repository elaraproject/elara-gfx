// use effi;
use gl;
use effi::*;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::{error::Error, ptr::null};
use std::any::Any;
// Re-export gl types so they can directly be
// used from the crate itself
pub use gl::types::*;
use std::ffi::{CStr, CString};

use crate::GLWindow;

const DUMMY_VS: &str = r#"
#version 330 core
attribute vec3 position;

void main()
{
    gl_Position = vec4(position, 1.0);
}
"#;

// A generic, temporary error type for encapsulating
// all GL errors until proper error handling can be implemented
pub type GfxResult = Result<(), Box<dyn Error>>;

pub fn null_ptr<T>() -> *const T {
    std::ptr::null()
}

pub fn sized_ptr<T>(size: usize) -> *const T {
    (size * std::mem::size_of::<T>()) as *const () as *const _
}

// From: https://users.rust-lang.org/t/how-check-type-of-variable/33845/6

pub fn gfxinfo() {
    if let Some(renderer_str) = glGetString(gl::RENDERER) {
        println!("OpenGL Renderer: {}", renderer_str);
    }
    if let Some(version_str) = glGetString(gl::VERSION) {
        println!("OpenGL Version: {}", version_str);
    }
    if let Some(glsl_version_str) = glGetString(gl::SHADING_LANGUAGE_VERSION) {
    println!("GLSL Version: {}", glsl_version_str);
    }
}

pub fn get_dummy_vs() -> &'static str {
    DUMMY_VS
}


fn set_attribute(program: GLuint, attrib_name: &str, size: i32, stride: i32, ptr: *const f32) {
    // Append null terminator to Rust-converted strings
    // so that they can be valid C strings passed to OpenGL
    let null_terminator: char = '\0';
    let mut attrib_name_bytes = attrib_name.as_bytes().to_vec();
    attrib_name_bytes.push(null_terminator as u8);
    unsafe {
        let attrib = gl::GetAttribLocation(
            program, attrib_name_bytes.as_ptr().cast());
        gl::VertexAttribPointer(
            attrib as GLuint,
            size,
            gl::FLOAT,
            0,
            stride * std::mem::size_of::<f32>() as GLsizei,
            ptr as *const c_void
        );
        gl::EnableVertexAttribArray(attrib as GLuint);
    }
}

fn set_default_uniforms(program: GLuint, window: &GLWindow<()>) {
    unsafe {
        let res_uniform = gl::GetUniformLocation(
            program,
            b"u_resolution\0".as_ptr().cast()
        );
        gl::Uniform2ui(res_uniform, 
            window.width() as u32, 
            window.height() as u32);
    }
}

// TODO try to implement generics
fn set_uniform_i32(program: GLuint, uniform_name: &str, uniform_params: &[i32]) -> Option<()>
{
    unsafe {
        let uniform_name = CString::new(uniform_name).unwrap();
        let uniform_location = gl::GetUniformLocation(program, uniform_name.as_ptr() as *const GLchar);
        match uniform_params.len() {
            1 => {
                gl::Uniform1i(uniform_location, uniform_params[0]);
            },
            2 => {
                gl::Uniform2i(uniform_location, uniform_params[0], uniform_params[1]);
            }
            3 => {
                gl::Uniform3i(uniform_location, uniform_params[0], uniform_params[1], uniform_params[2]);
            }
            4 => {
                gl::Uniform4i(uniform_location, uniform_params[0], uniform_params[1], uniform_params[2], uniform_params[3]);
            }
            _ => ()
        }
        Some(())
    }
}

fn set_uniform_u32(program: GLuint, uniform_name: &str, uniform_params: &[u32]) -> Option<()>
{
    unsafe {
        let uniform_name = CString::new(uniform_name).unwrap();
        let uniform_location = gl::GetUniformLocation(program, uniform_name.as_ptr() as *const GLchar);
        match uniform_params.len() {
            1 => {
                gl::Uniform1ui(uniform_location, uniform_params[0]);
            },
            2 => {
                gl::Uniform2ui(uniform_location, uniform_params[0], uniform_params[1]);
            }
            3 => {
                gl::Uniform3ui(uniform_location, uniform_params[0], uniform_params[1], uniform_params[2]);
            }
            4 => {
                gl::Uniform4ui(uniform_location, uniform_params[0], uniform_params[1], uniform_params[2], uniform_params[3]);
            }
            _ => ()
        }
        Some(())
    }
}

fn set_uniform_f32(program: GLuint, uniform_name: &str, uniform_params: &[f32]) -> Option<()>
{
    unsafe {
        let uniform_name = CString::new(uniform_name).unwrap();
        let uniform_location = gl::GetUniformLocation(program, uniform_name.as_ptr() as *const GLchar);
        match uniform_params.len() {
            1 => {
                gl::Uniform1f(uniform_location, uniform_params[0]);
            },
            2 => {
                gl::Uniform2f(uniform_location, uniform_params[0], uniform_params[1]);
            }
            3 => {
                gl::Uniform3f(uniform_location, uniform_params[0], uniform_params[1], uniform_params[2]);
            }
            4 => {
                gl::Uniform4f(uniform_location, uniform_params[0], uniform_params[1], uniform_params[2], uniform_params[3]);
            }
            _ => ()
        }
        Some(())
    }
}
pub struct DefaultRenderer {
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
}

#[derive(Debug)]
pub struct ElementRenderer {
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

pub trait Renderer {
    fn attribute(&self, attrib_name: &str, size: i32, stride: i32, ptr: *const f32) -> ();

    fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    /*
        Uniforms planned to be supported are identical to Shadertoy's:

        uniform vec2      u_resolution;           // viewport resolution (in pixels)
        uniform float     u_time;                 // shader playback time (in seconds)
        uniform float     u_timeDelta;            // render time (in seconds)
        uniform int       u_frame;                // shader playback frame
        uniform float     u_channelTime[4];       // channel playback time (in seconds)
        uniform vec3      u_channelResolution[4]; // channel resolution (in pixels)
        uniform vec4      u_mouse;                // mouse pixel coords. xy: current (if MLB down), zw: click
        uniform samplerXX u_channel0..3;          // input channel. XX = 2D/Cube
        uniform vec4      u_date;                 // (year, month, day, time in seconds)
        uniform float     u_sampleRate;           // sound sample rate (i.e., 44100)
    */

    fn uniform_i32(&self, uniform_name: &str, uniform_params: &[i32]);
    
    fn uniform_u32(&self, uniform_name: &str, uniform_params: &[u32]);
    
    fn uniform_f32(&self, uniform_name: &str, uniform_params: &[f32]);

    fn set_default_uniforms(&self, window: &GLWindow<()>);

    fn bg(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {    
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(r, g, b, a);
        }
    }

    fn draw_default(&self);
}

#[derive(Debug)]
pub struct PixelArray {
    width: i32,
    height: i32,
    channels: i32,
    data: Vec<u8>
}

impl PixelArray {
    pub fn new(width: i32, height: i32) -> PixelArray {
        PixelArray { 
            width: width,
            height: height,
            channels: 4, 
            data: Vec::new() }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data = data.to_vec();
    }

    pub fn write_ppm(&self) -> String {
        let mut data_str = String::new();
        for i in (0..self.data.len()).step_by(4) {
            for j in (0..3) {
                data_str += &(self.data[i + j].to_string() + " ");
            }
        }
        format!("P3\n# Created by elara-gfx\n{} {}\n255\n{}", self.width, self.height, data_str)
    }

    pub fn save_as_ppm(&self, path: PathBuf) -> GfxResult {
        let mut output = File::create(path)?;
        write!(output, "{}", self.write_ppm())?;
        Ok(())
    }
}

impl DefaultRenderer {
    pub fn new(vertices: Vec<f32>, vs_src: &str, fs_src: &str) -> Self {
        unsafe {
            let vertex_shader = _compile_shader(&vs_src, gl::VERTEX_SHADER);
            let fragment_shader = _compile_shader(&fs_src, gl::FRAGMENT_SHADER);

            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            gl::UseProgram(program);

            // Program compilation check
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut vbo = std::mem::zeroed();
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            Self { program, vao, vbo }
        }
    }

    pub fn draw(&self, first: i32, count: i32) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, first, count)
        }
    }
}

// pub fn cast_to_array<T>(array: &[T]) -> &'static [T]
//     where Vec<T>: FromIterator<T>
// {
//     let casted_vec: Vec<T> = array.iter().map(|&e| e as T).collect();
//     casted_vec.as_slice()
// }
impl Renderer for DefaultRenderer {
    fn attribute(&self, attrib_name: &str, size: i32, stride: i32, ptr: *const f32) {
        set_attribute(self.program, attrib_name, size, stride, ptr);
    }

    // TODO: replace with better generics
    fn uniform_i32(&self, uniform_name: &str, uniform_params: &[i32]) {
        set_uniform_i32(self.program, uniform_name, uniform_params);
    }
    
    fn uniform_u32(&self, uniform_name: &str, uniform_params: &[u32]) {
        set_uniform_u32(self.program, uniform_name, uniform_params);
    }

    fn uniform_f32(&self, uniform_name: &str, uniform_params: &[f32]) {
        set_uniform_f32(self.program, uniform_name, uniform_params);
    }

    fn set_default_uniforms(&self, window: &GLWindow<()>) -> () {
        set_default_uniforms(self.program, &window);
    }

    fn draw_default(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3)
        }
    }
}

impl Drop for DefaultRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl ElementRenderer {
    pub fn new(vertices: Vec<f32>, elements: Vec<i32>, vs_src: &str, fs_src: &str) -> Self {
        unsafe {
            let vertex_shader = _compile_shader(&vs_src, gl::VERTEX_SHADER);
            let fragment_shader = _compile_shader(&fs_src, gl::FRAGMENT_SHADER);

            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            gl::UseProgram(program);

            // Program compilation check
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut vbo = std::mem::zeroed();
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let mut ebo = std::mem::zeroed();
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (elements.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                elements.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            Self { program, vao, vbo, ebo }
        }
    }

    pub fn draw(&self, count: i32) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_INT, 0 as *const _);
        }
    }
}

impl Renderer for ElementRenderer {

    fn attribute(&self, attrib_name: &str, size: i32, stride: i32, ptr: *const f32) {
        set_attribute(self.program, attrib_name, size, stride, ptr);
    }

    // TODO: replace with better generics
    fn uniform_i32(&self, uniform_name: &str, uniform_params: &[i32]) {
        set_uniform_i32(self.program, uniform_name, uniform_params);
    }
    
    fn uniform_u32(&self, uniform_name: &str, uniform_params: &[u32]) {
        set_uniform_u32(self.program, uniform_name, uniform_params);
    }

    fn uniform_f32(&self, uniform_name: &str, uniform_params: &[f32]) {
        set_uniform_f32(self.program, uniform_name, uniform_params);
    }

    fn set_default_uniforms(&self, window: &GLWindow<()>) -> () {
        set_default_uniforms(self.program, &window);
    }

    fn draw_default(&self) {
        
    }
}

impl Drop for ElementRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

// Converts a Rust string to a OpenGL-compatible C string
pub fn to_gl_str(input_str: &str) -> *const GLchar {
    input_str.as_bytes().as_ptr().cast()
}

// Converts a OpenGL-compatible C string to a Rust strong
pub fn from_gl_str(input_str: *const GLchar) -> &'static str {
    unsafe {
        let res_c_str = CStr::from_ptr(input_str as *const i8);
        res_c_str.to_str().unwrap()
    }
}

pub fn _compile_shader(src: &str, shader_type: GLuint) -> GLuint {
    let shader = glCreateShader(shader_type);
    unsafe {
        gl::ShaderSource(
            shader, 
            1,
            &src.as_bytes().as_ptr().cast(),
            &src.len().try_into().unwrap(),
        );
        gl::CompileShader(shader);
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len: i32 = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut log = String::with_capacity(log_len as usize);
                log.extend(std::iter::repeat('\0').take(log_len as usize));
                gl::GetShaderInfoLog(
                    shader,
                    log_len,
                    &mut log_len,
                    (&log[..]).as_ptr() as *mut GLchar,
                );
                log.truncate(log_len as usize);
            panic!("Shader compile error: {}", log);
        }
    }
    shader
}


// OpenGL abstractions here
// Wrappers around OpenGL functions
// (WIP) plan is to wrap all the raw FFI functions to
// have proper error handling and be completely safe
pub fn glGetString(gl_str: GLenum) -> Option<&'static str> {
    unsafe {
        let s = gl::GetString(gl_str);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()).to_str().unwrap())
    }
}
pub fn glClearColor(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r as GLclampf, g as GLclampf, b as GLclampf, a as GLclampf);
    }
}

pub fn glClear(mask: GLbitfield) {
    unsafe {
        gl::Clear(mask);
    }
}

pub fn glGenVertexArrays(n: i32, arrays: u32) {
    unsafe {
        gl::GenVertexArrays(n, arrays as *mut u32);
    }
}

pub fn glBindVertexArray(array: u32) {
    unsafe {
        gl::BindVertexArray(array);
    }
}

pub fn glGenBuffers(n: i32, arrays: u32) {
    unsafe {
        gl::GenBuffers(n, arrays as *mut u32);
    }
}

pub fn glBindBuffer(target: u32, buffer: u32) {
    unsafe {
        gl::BindBuffer(target, buffer);
    }
}

pub fn glBufferData(target: u32, size: usize, data: &[f32], usage: u32) {
    unsafe { gl::BufferData(target, size as isize, data.as_ptr().cast(), usage) }
}

pub fn glCreateShader(shader_type: GLenum) -> GLuint {
    unsafe { gl::CreateShader(shader_type) }
}

pub fn glShaderSource(shader: u32, count: i32, shader_src: &str, len: i32) {
    unsafe {
        let shader_cstr = CString::new(shader_src).unwrap();
        let mut shader_cstr_ptr = shader_cstr.as_ptr() as *const i8;
        gl::ShaderSource(shader, count, &mut shader_cstr_ptr, len as *mut i32)
    }
}

pub fn glCompileShader(shader: GLuint) {
    unsafe {
        gl::CompileShader(shader);
    }
}

pub fn glCreateProgram() -> GLuint {
    unsafe { gl::CreateProgram() }
}

pub fn glAttachShader(program: GLuint, shader: GLuint) {
    unsafe {
        gl::AttachShader(program, shader);
    }
}

pub fn glBindFragDataLocation(program: GLuint, num: u32, name: &str) {
    unsafe {
        gl::BindFragDataLocation(program, num, name.as_ptr() as *const i8);
    }
}

pub fn glLinkProgram(program: GLuint) {
    unsafe {
        gl::LinkProgram(program);
    }
}

pub fn glUseProgram(program: GLuint) {
    unsafe {
        gl::UseProgram(program);
    }
}

pub fn glGetAttribLocation(program: GLuint, attrib: &str) -> GLint {
    unsafe { gl::GetAttribLocation(program, attrib.as_ptr() as *const i8) }
}

pub fn glEnableVertexAttribArray(attrib: GLint) {
    unsafe {
        gl::EnableVertexAttribArray(attrib as u32);
    }
}

pub fn glVertexAttribPointer(
    attrib: GLint,
    size: i32,
    ptr_type: GLenum,
    normalized: bool,
    stride: GLsizei,
    ptr: i32,
) {
    unsafe {
        // gl::VertexAttribPointer(attrib as u32, size, ptr_type, normalized as u8, /* i32 */, ptr as *const GLvoid)
        gl::VertexAttribPointer(
            attrib as u32,
            size,
            ptr_type,
            normalized as u8,
            stride,
            ptr as *const GLvoid,
        )
    }
}

pub fn glDrawArrays(mode: GLenum, first: i32, count: i32) {
    unsafe {
        gl::DrawArrays(mode, first, count);
    }
}