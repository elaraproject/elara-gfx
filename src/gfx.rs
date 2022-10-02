mod effi;
use gl;
// Re-export gl types so they can directly be
// used from the crate itself
pub use gl::types::*;
use std::ffi::CString;

// OpenGL abstractions here
// Wrappers around OpenGL functions
// (WIP) plan is to wrap all the raw FFI functions to
// have proper error handling and be completely safe
pub fn glGetString(gl_str: GLenum) -> &'static str {
    unsafe {
        let res_ptr = gl::GetString(gl_str);
        let res_c_str = CStr::from_ptr(res_ptr as *const i8);
        res_c_str.to_str().unwrap()
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
        gl::ShaderSource(
            shader,
            count,
            &mut shader_cstr_ptr,
            len as *mut i32,
        )
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
    ptr: i32,
) {
    unsafe {
        gl::VertexAttribPointer(
            attrib as u32,
            size,
            ptr_type,
            normalized as u8,
            ptr as *const GLvoid,
        )
    }
}

pub fn glDrawArrays(mode: GLenum, first: i32, count: i32) {
    unsafe {
        gl::DrawArrays(mode, first, count);
    }
}
