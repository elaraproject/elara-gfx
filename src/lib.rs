#![allow(non_snake_case)]

use raw_gl_context::{GlConfig, GlContext};
use std::marker::PhantomData;
use winit::dpi::LogicalSize;
pub use winit::event::Event;
pub use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
pub mod gfx;

pub struct GLWindow<T: 'static> {
    width: i32,
    height: i32,
    title: String,
    base_window: Window,
    context: GlContext,
    phantom: PhantomData<T>,
}

impl<T> GLWindow<T> {
    pub fn new(width: i32, height: i32, title: &str, event_loop: &EventLoop<T>) -> GLWindow<T> {
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let context = GlContext::create(&window, GlConfig::default()).unwrap();
        GLWindow {
            width,
            height,
            title: String::from(title),
            base_window: window,
            context,
            phantom: PhantomData,
        }
    }

    pub fn init_gl(&self) {
        self.base_window.set_title(self.title.as_str());
        self.base_window
            .set_inner_size(LogicalSize::new(self.width, self.height));
        self.context.make_current();
        gl::load_with(|symbol| self.context.get_proc_address(symbol) as *const _);
    }

    pub fn clear(&self) {
        gfx::glClear(gl::COLOR_BUFFER_BIT);
    }

    pub fn render(&self, render_func: &dyn Fn()) {
        self.context.make_current();
        render_func();
        self.context.swap_buffers();
        self.context.make_not_current();
    }

    pub fn redraw(&self) {
        self.context.make_current();
        gfx::glClearColor(1.0, 1.0, 1.0, 1.0);
        gfx::glClear(gl::COLOR_BUFFER_BIT);
        self.context.swap_buffers();
        self.context.make_not_current();
    }
}
