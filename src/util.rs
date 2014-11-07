
use std::str::{Slice,Owned};

use gl;

pub fn check_error(file: &str, line: uint) {
    let err_code = gl::GetError();
    if err_code != 0 {
        let message = match err_code {
            gl::INVALID_ENUM => "GL_INVALID_ENUM",
            gl::INVALID_VALUE => "GL_INVALID_VALUE",
            gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
            gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
            gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
            // gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
            // gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
            _ => "Unrecognized error code"
        };
        fail!("OpenGL Error: {} ({}) at {}:{}", message, err_code, file, line);
    }
}

/// Always shorten the vector to exclude the null byte!
pub fn vec_to_string(vec: Vec<u8>) -> String {
    match String::from_utf8(vec) {
        Ok(string) => string,
        Err(vec) => match String::from_utf8_lossy(vec[]) {
            Owned(string) => string,
            Slice(str_slice) => String::from_str(str_slice) // This one shouldn't probably happen
        }
    }
}