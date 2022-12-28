//! All error types for elara-gfx
use std::error::Error;
use std::fmt;

// pub type WindowResult<T> = Result<T, WindowError>;

#[derive(Debug)]
pub enum WindowError {
    WindowCreationError,
    PlatformError,
}

// TODO: proper Error trait implementation
impl Error for WindowError {}

// TODO: proper error implementation for `WindowError`
impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred during window creation.")
    }
}

pub type GlResult<T> = Result<T, GlError>;

#[derive(Debug)]
pub struct GlError;

// TODO: proper Error trait implementation
impl Error for GlError {}

// TODO: proper error implementation for `GlError`
impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred with OpenGL.")
    }
}
