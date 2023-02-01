pub use gl::types::*;
pub use std::ffi::*;
pub fn null_ptr<T>() -> *const T {
    std::ptr::null()
}
