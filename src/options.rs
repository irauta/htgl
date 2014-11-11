
use gl;
use gl::types::GLenum;

use super::RenderOption;

pub fn set_option(option: RenderOption) {
    match option {
        super::ClearColor(r, g, b, a) => unsafe { gl::ClearColor(r, g, b, a) },
        super::DepthTest(enable) => set_capability(gl::DEPTH_TEST, enable),
        super::CullingEnabled(enable) => set_capability(gl::CULL_FACE, enable)
    }
}

fn set_capability(cap: GLenum, enable: bool) {
    if enable {
        unsafe {
            gl::Enable(cap);
        }
    }
    else {
        unsafe {
            gl::Disable(cap);
        }
    }
}