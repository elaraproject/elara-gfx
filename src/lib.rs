#![allow(non_snake_case)]

use canvas::Canvas;
pub use gl;
use raw_gl_context::{GlConfig, GlContext};
use std::ffi::{CStr, CString};
use winit::dpi::PhysicalSize;
pub use winit::event::Event;
pub use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
// pub mod gfx;
mod error;
pub mod types;
use elara_log::prelude::*;
use error::*;
mod image;
pub use image::{PixelArray, RGBA};
pub mod canvas;

// NOTE: elara-gfx uses elara-log internally to log
// errors, if elara-log is not initialized
// library errors will not show up!

pub fn gl_get_string(gl_str: types::GLenum) -> Option<&'static str> {
    unsafe {
        let s = gl::GetString(gl_str);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()).to_str().unwrap())
    }
}

// Reference: https://github.com/jminer/clear-coat/blob/068f247ce84017583cc49a257d84e659137e6c4f/src/attributes.rs#L17
pub fn to_cstr(s: &'static str) -> *const types::c_char {
    if s.as_bytes().last() == Some(&0) && !s.as_bytes()[..s.len() - 1].contains(&b'\0') {
        s.as_bytes().as_ptr() as *const types::c_char
    } else {
        let c_str = CString::new(s).unwrap();
        c_str.as_ptr() as *const types::c_char
    }
}

pub fn from_cstr(s: *const types::c_char) -> Option<&'static str> {
    unsafe { (!s.is_null()).then(|| CStr::from_ptr(s.cast()).to_str().unwrap()) }
}

pub fn gl_info() {
    if let Some(renderer_str) = gl_get_string(gl::RENDERER) {
        info!("[elara-gfx] OpenGL Renderer: {}", renderer_str);
    }
    if let Some(version_str) = gl_get_string(gl::VERSION) {
        info!("[elara-gfx] OpenGL Version: {}", version_str);
    }
    if let Some(glsl_version_str) = gl_get_string(gl::SHADING_LANGUAGE_VERSION) {
        info!("[elara-gfx] GLSL Version: {}", glsl_version_str);
    }
}

pub fn check_gl_error() {
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


// Temporary: all WindowHandler errors use strings
pub type HandlerResult<T> = Result<T, String>;

// Temporary: type for defining pointer-obtainable types
pub trait PointerType: Sized {
    type CType;
    fn as_ptr(&self) -> *mut Self::CType;
}

pub trait WindowHandler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        Ok(())
    }

    fn post_draw(&mut self) -> HandlerResult<()> {
    	Ok(())
    }
    fn on_resize(&mut self) {}

    fn save_rendering(&self, width: i32, height: i32) -> Option<PixelArray> {
    	// source: https://lencerf.github.io/post/2019-09-21-save-the-opengl-rendering-to-image-file/
        let n_channels = 4;
        let mut stride = n_channels * width;
        stride += if stride % 4 != 0 { 4 - stride % 4 } else { 0 };
        let buffer_size = stride * height;
        let buffer = vec![0; buffer_size as usize];
        unsafe {
	        gl::PixelStorei(gl::PACK_ALIGNMENT, 4);
	        gl::ReadBuffer(gl::FRONT);
	        gl::ReadPixels(0, 0, width, height, gl::RGBA, gl::UNSIGNED_BYTE, buffer.as_ptr() as *mut u8 as *mut types::c_void);
        }
        PixelArray::from_bytearray(buffer, width as usize, height as usize)
    }
    // TODO: add other methods such as on_mouse_move(), on_keydown(),
    // on_click(), on_cursor_move() for handling on non-draw events
}

pub trait Draw {
    fn draw(&mut self, win: &GLWindow) -> Result<Canvas, String> {
        Ok(Canvas::new(win))
    }
}

#[derive(Debug)]
pub struct GLWindowHandler {
    event_loop: EventLoop<()>,
}

impl GLWindowHandler {
    pub fn new() -> GLWindowHandler {
        GLWindowHandler {
            event_loop: EventLoop::new(),
        }
    }

    pub fn run_loop<H>(self, window: GLWindow, mut handler: H)
    where
        H: WindowHandler + 'static,
    {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    info!("[elara-gfx] Close request received, exiting...");
                    control_flow.set_exit();
                }
                Event::MainEventsCleared => {
                }
                Event::RedrawRequested(_) => {
                    // Render function
                    window.make_current();
                    handler.on_draw().unwrap();
                    window.swap_buffers();
                    handler.post_draw().unwrap();
                    window.make_not_current();
                }
                _ => {}
            }
        });
    }
}

impl Default for GLWindowHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GLWindow {
    width: i32,
    height: i32,
    base_window: Window,
    is_visible: bool,
    context: GlContext,
}

pub struct WindowOptions {
    pub title: &'static str,
    pub width: i32,
    pub height: i32,
    pub is_visible: bool,
}

impl WindowOptions {
    pub fn new(title: &'static str, width: i32, height: i32, is_visible: bool) -> WindowOptions {
        WindowOptions {
            title,
            width,
            height,
            is_visible,
        }
    }
}

impl Default for WindowOptions {
    fn default() -> WindowOptions {
        WindowOptions {
            title: "OpenGL window",
            width: 1200,
            height: 900,
            is_visible: true,
        }
    }
}

impl GLWindow {
    pub fn new(opts: WindowOptions) -> Result<(GLWindowHandler, GLWindow), WindowError> {
        let window_handler = GLWindowHandler::new();
        let window = WindowBuilder::new()
            .with_title(opts.title)
            .with_inner_size(PhysicalSize::new(opts.width, opts.height))
            .with_visible(opts.is_visible)
            .build(&window_handler.event_loop)
            .unwrap();
        // Enable MSAA
        let config = GlConfig { 
            samples: Some(4), 
            ..GlConfig::default()
        };
        let context = GlContext::create(&window, config).unwrap();
        
        let gl_window = GLWindow {
            width: opts.width,
            height: opts.height,
            base_window: window,
            is_visible: opts.is_visible,
            context,
        };
        Ok((window_handler, gl_window))
    }

    pub fn new_with_title(title: &'static str) -> Result<(GLWindowHandler, GLWindow), WindowError> {
        let opts = WindowOptions {
            title,
            ..WindowOptions::default()
        };
        Self::new(opts)
    }

    pub fn new_sized(title: &'static str, width: i32, height: i32) -> Result<(GLWindowHandler, GLWindow), WindowError> {
        let opts = WindowOptions {
            title,
            width, 
            height,
            ..WindowOptions::default()
        };
        Self::new(opts)
    }

    pub fn get_context(&self) -> GlResult<()> {
        self.context.make_current();
        gl::load_with(|symbol| self.context.get_proc_address(symbol) as *const types::c_void);
        // Enable MSAA
        unsafe { 
            gl::Enable(gl::MULTISAMPLE);
        }
        Ok(())
    }

    pub fn set_visible(&mut self, visibility: bool) {
        self.is_visible = visibility;
        self.base_window.set_visible(visibility);
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn inner_width(&self) -> i32 {
        self.base_window.inner_size().width as i32
    }

    pub fn inner_height(&self) -> i32 {
        self.base_window.inner_size().width as i32
    }

    pub fn set_title(&mut self, title: &str) {
        self.base_window.set_title(title);
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn make_current(&self) {
        self.context.make_current();
    }

    pub fn make_not_current(&self) {
        self.context.make_not_current();
    }

    pub fn swap_buffers(&self) {
        self.context.swap_buffers();
    }

    pub fn render(&self, render_func: &dyn Fn()) {
        self.context.make_current();
        render_func();
        self.context.swap_buffers();
        self.context.make_not_current();
    }

    pub fn redraw(&self, redraw_func: &dyn Fn()) {
        self.context.make_current();
        redraw_func();
        self.context.swap_buffers();
        self.context.make_not_current();
    }
}

pub struct Texture2D(pub types::GLuint);

impl Texture2D {
    pub fn new() -> Result<Texture2D, String> {
        let mut texture = 0;
        unsafe { gl::GenTextures(1, &mut texture) };
        if texture != 0 {
            Ok(Texture2D(texture))
        } else {
            let err = String::from("Texture2D creation failed");
            Err(format!("[elara-gfx] {}", err))
        }
    }
    
    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.0) }
    }
    
    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
    
    pub fn parameter_2d(&self, pname: types::GLenum, param: types::GLint) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, pname, param);
        }
    }
    
    pub fn enable_alpha_blend(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }
    
    pub fn generate_mipmap(&self) {
        unsafe {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
    
    pub fn set_image_2d(&self, img: PixelArray) {
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           img.width as i32,
                           img.height as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           img.data().as_ptr() as *const u8 as *const types::c_void);
        }
    }
}

pub struct VertexArray(pub types::GLuint);

impl VertexArray {
    pub fn new() -> Result<VertexArray, String> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Ok(VertexArray(vao))
        } else {
            let err = String::from("Vertex array creation failed");
            Err(format!("[elara-gfx] {}", err))
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.0) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn vertex_attrib_pointer<T>(
        &self,
        idx: types::GLuint,
        size: types::GLint,
        arr_type: types::GLenum,
        normalized: bool,
        stride: usize,
        pointer: usize
    ) {
        unsafe {
            gl::VertexAttribPointer(
                idx,
                size,
                arr_type as types::GLenum,
                normalized as types::GLboolean,
                (stride * std::mem::size_of::<T>()) as types::GLsizei,
                (pointer * std::mem::size_of::<T>()) as *const types::c_void,
            )
        }
    }

    pub fn enable_vertex_attrib(&self, index: types::GLuint) {
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str) -> i32 {
        unsafe {
            let cstr = CString::new(name).unwrap();
            gl::GetAttribLocation(program.clone().id(), cstr.as_ptr())
        }
    }
}

pub struct Buffer(pub types::GLuint);

impl Buffer {
    pub fn new() -> Result<Buffer, String> {
        let mut buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
        }
        if buffer != 0 {
            Ok(Buffer(buffer))
        } else {
            let err = String::from("Buffer creation failed");
            Err(format!("[elara-gfx] {}", err))
        }
    }

    pub fn bind(&self, buffer_type: BufferType) {
        unsafe { gl::BindBuffer(buffer_type as types::GLenum, self.0) }
    }

    pub fn unbind(&self, buffer_type: BufferType) {
        unsafe { gl::BindBuffer(buffer_type as types::GLenum, 0) }
    }

    // TODO: this might be better using generic instead of assuming
    // data to be an array of f32 values
    pub fn data<T>(&self, buffer_type: BufferType, data: &[T], usage: types::GLenum) {
        unsafe {
            gl::BufferData(
                buffer_type as types::GLenum,
                (data.len() * std::mem::size_of::<T>()) as types::GLsizeiptr,
                std::mem::transmute(&data[0]),
                usage,
            )
        }
    }

    // Binds a buffer object and allocates memory without copying
    // data to buffer
    pub fn data_empty<T>(&self, buffer_type: BufferType, size: usize, usage: types::GLenum) {
        unsafe {
            gl::BufferData(
                buffer_type as types::GLenum,
                (size * std::mem::size_of::<T>()) as types::GLsizeiptr,
                std::ptr::null(),
                usage
            )
        }
    }

    pub fn subdata<T, const N: usize>(&self, buffer_type: BufferType, offset: isize, data: &[T; N]) 
    {
        unsafe {
            gl::BufferSubData(
                buffer_type as types::GLenum,
                offset,
                std::mem::size_of::<[T; N]>() as isize,
                data.as_ptr() as *const types::c_void
            )
        }
    }
}

pub struct Uniform(pub types::GLint);

impl Uniform {
    pub fn new(program: &Program, uniform_name: &str) -> Result<Uniform, String> {
        let uniform_name = CString::new(uniform_name).unwrap();
        let id = unsafe { gl::GetUniformLocation(program.id(), uniform_name.as_ptr().cast()) };
        Ok(Uniform(id))
    }

    pub fn id(&self) -> types::GLint {
        self.0
    }
    
    pub fn uniform1f(&self, val: f32) {
        unsafe {
            gl::Uniform1f(self.0, val);
        }
    }
    
    pub fn uniform2f(&self, val_a: f32, val_b: f32) {
        unsafe {
            gl::Uniform2f(self.0, val_a, val_b);
        }
    }

    pub fn uniform3f(&self, val_a: f32, val_b: f32, val_c: f32) {
        unsafe {
            gl::Uniform3f(self.0, val_a, val_b, val_c);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Array = gl::ARRAY_BUFFER as isize,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
    FrameBuffer = gl::FRAMEBUFFER as isize
}

pub struct FrameBuffer(pub types::GLuint);

impl FrameBuffer {
    pub fn new() -> Result<FrameBuffer, String> {
        let mut framebuffer = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer);
        }
        if framebuffer != 0 {
            Ok(FrameBuffer(framebuffer))
        } else {
            let err = String::from("Framebuffer creation failed");
            Err(format!("[elara-gfx] {}", err))
        }
    }

    pub fn bind(&self) {
        unsafe { 
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.0);
        }
    }
    
    pub fn set_renderbuffer(&self, render_buffer: RenderBuffer) {
        unsafe {
            gl::FramebufferRenderbuffer(
                gl::DRAW_FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::RENDERBUFFER,
                render_buffer.id()
            )
        }
    }
    
    pub fn set_color_buffer(&self) {
        unsafe {
            gl::ReadBuffer(gl::COLOR_ATTACHMENT0);
        }
    }
}

pub struct RenderBuffer(pub types::GLuint);

impl RenderBuffer {
    pub fn new() -> Result<RenderBuffer, String> {
        let mut render_buffer = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut render_buffer);
        }
        if render_buffer != 0 {
            Ok(RenderBuffer(render_buffer))
        } else {
            let err = String::from("Framebuffer creation failed");
            Err(format!("[elara-gfx] {}", err))
        }
    }

    pub fn bind(&self) {
        unsafe { 
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.0);
        }
    }
    
    pub fn id(&self) -> u32 {
        self.0
    }
    
    pub fn set_storage(&self, width: i32, height: i32) {
        unsafe {
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::RGB565, width, height);
        }
    }
}

pub struct Shader {
    id: types::GLuint,
}

impl Shader {
    pub fn new(source: &str, shader_type: types::GLenum) -> Result<Shader, String> {
        let id = create_shader(source, shader_type)?;
        Ok(Shader { id })
    }

    pub fn id(&self) -> types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

#[derive(Clone)]
pub struct Program {
    id: types::GLuint,
}

impl Program {
    pub fn new(shaders: &[Shader]) -> Result<Program, String> {
        let id = create_program(shaders)?;
        Ok(Program { id })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unuse_program(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_attribute(&self, attrib_name: &str, size: i32, stride: i32, ptr: *const f32) {
        set_attribute(self.id, attrib_name, size, stride, ptr);
    }

    pub fn id(&self) -> types::GLuint {
        self.id
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn set_attribute(
    program: types::GLuint,
    attrib_name: &str,
    size: i32,
    stride: i32,
    ptr: *const f32,
) {
    // Append null terminator to Rust-converted strings
    // so that they can be valid C strings passed to OpenGL
    let null_terminator: char = '\0';
    let mut attrib_name_bytes = attrib_name.as_bytes().to_vec();
    attrib_name_bytes.push(null_terminator as u8);
    unsafe {
        let attrib = gl::GetAttribLocation(program, attrib_name_bytes.as_ptr().cast());
        gl::VertexAttribPointer(
            attrib as types::GLuint,
            size,
            gl::FLOAT,
            0,
            stride * std::mem::size_of::<f32>() as types::GLsizei,
            ptr as *const types::c_void,
        );
        gl::EnableVertexAttribArray(attrib as types::GLuint);
    }
}

pub fn create_vao() -> types::GLuint {
    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };
    vao
}

pub fn create_vbo() -> types::GLuint {
    let mut vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vbo) };
    vbo
}

pub fn create_ebo() -> types::GLuint {
    let mut ebo = 0;
    unsafe { gl::GenBuffers(1, &mut ebo) };
    ebo
}

pub fn create_fbo() -> types::GLuint {
    let mut fbo = 0;
    unsafe { gl::GenFramebuffers(1, &mut fbo) };
    fbo
}

fn create_shader(source: &str, shader_type: types::GLenum) -> Result<types::GLuint, String> {
    let id = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(
            id,
            1,
            &source.as_bytes().as_ptr().cast(),
            &source.len().try_into().unwrap(),
        );
        gl::CompileShader(id);

        let mut success = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            if shader_type == gl::VERTEX_SHADER {
                warn!("[elara-gfx] Vertex shader compilation failed");
            } else {
                warn!("[elara-gfx] Fragment shader compilation failed");
            }
            let mut log_len = 0_i32;
            let mut error: Vec<u8> = Vec::with_capacity(gl::INFO_LOG_LENGTH as usize);
            gl::GetShaderInfoLog(
                id,
                gl::INFO_LOG_LENGTH as i32,
                &mut log_len,
                error.as_mut_ptr().cast(),
            );
            error.set_len(log_len.try_into().unwrap());
            let error_msg = String::from_utf8_lossy(&error);
            return Err(error_msg.to_string());
        }
        Ok(id)
    }
}

fn create_program(shaders: &[Shader]) -> Result<types::GLuint, String> {
    let id = unsafe { gl::CreateProgram() };
    for shader in shaders {
        unsafe { gl::AttachShader(id, shader.id()) }
    }

    unsafe {
        gl::LinkProgram(id);
    }

    let mut success = 0;
    unsafe {
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

        if success == 0 {
            warn!("[elara-gfx] Program compilation failed");
            let mut log_len = 0_i32;
            let mut error: Vec<u8> = Vec::with_capacity(gl::INFO_LOG_LENGTH as usize);
            gl::GetProgramInfoLog(
                id,
                gl::INFO_LOG_LENGTH as i32,
                &mut log_len,
                error.as_mut_ptr().cast(),
            );
            error.set_len(log_len.try_into().unwrap());
            let error_msg = String::from_utf8_lossy(&error);
            return Err(error_msg.to_string());
        }

        for shader in shaders {
            gl::DetachShader(id, shader.id())
        }
    }

    Ok(id)
}
