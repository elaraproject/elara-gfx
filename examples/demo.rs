use elara_gfx::{GLWindow, EventLoop, ControlFlow};
use elara_gfx::gfx::*;

// const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
// const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

fn main() {
    let event_loop = EventLoop::new();
    let window = GLWindow::new(900, 600, "Window 1", &event_loop);
    window.init_gl();
    println!("OpenGL Renderer: {}", glGetString(gl::RENDERER));
    println!("OpenGL Version: {}", glGetString(gl::VERSION));
    println!("GLSL Version: {}", glGetString(gl::SHADING_LANGUAGE_VERSION));
    // Event handling
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            winit::event::Event::RedrawRequested(_) => {
                window.redraw();
            }
            _ => {
            }
        }
    });
}
