use raw_gl_context::{GlConfig, GlContext};

use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};

pub struct GLWindow {
    width: i32,
    height: i32,
    title: String,
    event_loop: EventLoop<T>,
    base_window: Window,
    context: GlContext
}


impl GLWindow {
    pub fn new(width: i32, height: i32, title: &str) -> GLWindow {
        // To prevent memory errors we have to make a dedicated cstring
        // to pass to tigrWindow
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let context = unsafe { 
        GlContext::create(&window, GlConfig::default()).unwrap()
        };
        GLWindow {
            width: width,
            height: height,
            title: String::from(title),
            event_loop: event_loop,
            base_window: window
        }
    }

    pub fn init_gl(&self) {
        unsafe {
            self.context.make_current();
        }
        gl::load_with(|symbol| self.context.get_proc_address(symbol) as *const _);
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
    
    pub fn redraw(&self) {
        unsafe {
            self.context.make_current();
        }

        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.context.swap_buffers();

        unsafe {
            self.context.make_not_current();
        }
    }
    pub fn handle_events(&self) {
        self.event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            winit::event::Event::RedrawRequested(_) => {
                self.redraw();
            }
            _ => {}
        }
    });
    }
}
