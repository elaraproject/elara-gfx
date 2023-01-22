#![allow(non_snake_case)]

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

// Temporary: all WindowHandler errors use strings
pub type HandlerResult<T> = Result<T, String>;

pub trait WindowHandler {
    fn on_draw(&mut self) -> HandlerResult<()> {
        Ok(())
    }
    fn on_resize(&mut self) {}
    // TODO: add other methods such as on_mouse_move(), on_keydown(),
    // on_click(), on_cursor_move() for handling on non-draw events
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
                    // Render function
                    window.make_current();
                    handler.on_draw().unwrap();
                    window.swap_buffers();
                    window.make_not_current();
                }
                Event::RedrawRequested(_) => {
                }
                _ => {
                }
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
            width: 900,
            height: 600,
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
        let context = GlContext::create(&window, GlConfig::default()).unwrap();
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

    pub fn get_context(&self) -> GlResult<()> {
        self.context.make_current();
        gl::load_with(|symbol| self.context.get_proc_address(symbol) as *const types::c_void);
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

    pub fn id(&self) -> types::GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
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
            warn!("[elara-gfx] Shader compilation failed");
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
