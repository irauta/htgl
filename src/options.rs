// Copyright 2015 Ilkka Rauta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module defined basic options or states that can be set in the GL context.
//! Examples are clear color and back face culling. See `RenderOption`.

use gl;
use gl::types::GLenum;

/// Rendering options.
pub enum RenderOption {
    /// glClearColor
    ClearColor(f32, f32, f32, f32),
    /// GL_DEPTH_TEST
    DepthTest(bool),
    /// GL_CULL_FACE
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