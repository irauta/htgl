
use gl;

pub fn check_error(file: &str, line: u32) {
    let err_code = unsafe { gl::GetError() };
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
        panic!("OpenGL Error: {} ({}) at {}:{}", message, err_code, file, line);
    }
}

/// Always remember to shorten the vector to exclude the null byte before passing the Vec to this fn!
pub fn vec_to_string(vec: Vec<u8>) -> String {
    match String::from_utf8(vec) {
        Ok(string) => string,
        Err(err) => slice_to_string(&err.into_bytes()[..])
    }
}

pub fn slice_to_string(slice: &[u8]) -> String {
    String::from_utf8_lossy(slice).into_owned()
}