
use gl;

pub fn check_error() {
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
        println!("Error happened! Error: {} ({})", message, err_code);
        fail!();
    }
}
