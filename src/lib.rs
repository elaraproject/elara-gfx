#![allow(non_snake_case)]

pub use gl;
use gl::types::*;
use raw_gl_context::{GlConfig, GlContext};
use std::ffi::CStr;
use winit::dpi::PhysicalSize;
pub use winit::event::Event;
pub use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
// pub mod gfx;
mod error;
mod types;
use error::*;

// TODO: use elara-log here instead once double-initialization
// problem is fixed
macro_rules! log {
    ($($arg:tt)+) => (println!("[elara-gfx] {}", (format!($($arg)+))));
}

pub fn gl_get_string(gl_str: GLenum) -> Option<&'static str> {
    unsafe {
        let s = gl::GetString(gl_str);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()).to_str().unwrap())
    }
}

pub fn gl_info() {
    if let Some(renderer_str) = gl_get_string(gl::RENDERER) {
        log!("OpenGL Renderer: {}", renderer_str);
    }
    if let Some(version_str) = gl_get_string(gl::VERSION) {
        log!("OpenGL Version: {}", version_str);
    }
    if let Some(glsl_version_str) = gl_get_string(gl::SHADING_LANGUAGE_VERSION) {
        log!("GLSL Version: {}", glsl_version_str);
    }
}

pub trait WindowHandler {
    fn on_draw(&self);
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

    pub fn run_loop(self, window: GLWindow, handler: &'static dyn WindowHandler) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    log!("Close request received, exiting...");
                    control_flow.set_exit();
                }
                Event::MainEventsCleared => {
                    // Render function
                    window.make_current();
                    unsafe { gl::Viewport(0, 0, window.width(), window.height()); }
                    handler.on_draw();
                    window.swap_buffers();
                    window.make_not_current();
                }
                Event::RedrawRequested(_) => {}
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
