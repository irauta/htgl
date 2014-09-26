
use gl;
use gl::types::GLenum;

pub enum RenderOption {
    ClearColor(f32, f32, f32, f32),
    DepthTest(bool)
}

pub fn set_option(option: RenderOption) {
    match option {
        ClearColor(r, g, b, a) => gl::ClearColor(r, g, b, a),
        DepthTest(enable) => set_capability(gl::DEPTH_TEST, enable)
    }
}

fn set_capability(cap: GLenum, enable: bool) {
    if enable {
        gl::Enable(cap);
    }
    else {
        gl::Disable(cap);
    }
}