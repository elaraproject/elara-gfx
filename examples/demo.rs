use elara_gfx::GLWindow;
use elara_gfx::gfx::*;

// const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
// const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

fn main() {
    let window: GLWindow = GLWindow::new(320, 240, "Window 1");
    window.init_gl();
    println!("OpenGL Renderer: {}", glGetString(GL_RENDERER));
    println!("OpenGL Version: {}", glGetString(GL_VERSION));
    println!("GLSL Version: {}", glGetString(GL_SHADING_LANGUAGE_VERSION));
    window.event_handler();
}
