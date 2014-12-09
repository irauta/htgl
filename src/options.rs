
use gl;
use gl::types::GLenum;

pub enum RenderOption {
    ClearColor(f32, f32, f32, f32),
    DepthTest(bool),
    CullingEnabled(bool)
}

pub fn set_option(option: RenderOption) {
    match option {
        RenderOption::ClearColor(r, g, b, a) => unsafe { gl::ClearColor(r, g, b, a) },
        RenderOption::DepthTest(enable) => set_capability(gl::DEPTH_TEST, enable),
        RenderOption::CullingEnabled(enable) => set_capability(gl::CULL_FACE, enable)
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