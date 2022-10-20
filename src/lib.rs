#![allow(non_snake_case)]

use raw_gl_context::{GlConfig, GlContext};
use std::marker::PhantomData;
use winit::dpi::PhysicalSize;
pub use winit::event::Event;
pub use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
pub mod gfx;

pub struct GLWindow<T: 'static> {
    width: i32,
    height: i32,
    title: String,
    base_window: Window,
    is_visible: bool,
    context: GlContext,
    phantom: PhantomData<T>,
}

impl<T> GLWindow<T> {
    pub fn new(width: i32, height: i32, title: &str, visibility: bool, event_loop: &EventLoop<T>) -> GLWindow<T> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height))
            .with_visible(visibility)
            .build(&event_loop).unwrap();
        let context = GlContext::create(&window, GlConfig::default()).unwrap();
        GLWindow {
            width,
            height,
            title: String::from(title),
            base_window: window,
            is_visible: visibility,
            context,
            phantom: PhantomData,
        }
    }

    pub fn init_gl(&self) {
        self.context.make_current();
        gl::load_with(|symbol| self.context.get_proc_address(symbol) as *const _);
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

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.base_window.set_title(title);
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        gfx::glClearColor(r, g, b, a);
        gfx::glClear(gl::COLOR_BUFFER_BIT);
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
