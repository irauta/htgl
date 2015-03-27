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

//! OpenGL context related information structures are defined in this module. Practically a little
//! more structured approach than a long list of glGet* results. See `ContextInfo`, it is the
//! "root" of context info structures.

use gl;
use gl::types::{GLint,GLenum};

/// Currently `ContextInfo` doesn't contain much. The fields act as "categories". See field
/// comments for further info.
#[derive(Debug)]
pub struct ContextInfo {
    /// Information related to uniform buffers.
    pub uniform_buffer: UniformBufferInfo
}

/// Information related to uniform buffers.
#[derive(Debug)]
pub struct UniformBufferInfo {
    /// GL_MAX_UNIFORM_BUFFER_BINDINGS
    pub max_bindings: GLint,
    /// GL_MAX_GEOMETRY_UNIFORM_BLOCKS
    pub max_geometry_blocks: GLint,
    /// GL_MAX_VERTEX_UNIFORM_BLOCKS
    pub max_vertex_blocks: GLint,
    /// GL_MAX_FRAGMENT_UNIFORM_BLOCKS
    pub max_fragment_blocks: GLint,
    /// GL_MAX_UNIFORM_BLOCK_SIZE
    pub max_block_size: GLint,
    /// GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT
    pub offset_alignment: GLint
}

/// Constructor for the context info. Causes a lof of glGet* calls!
pub fn build_info() -> ContextInfo {
    ContextInfo {
        uniform_buffer: UniformBufferInfo {
            max_bindings: get_integer(gl::MAX_UNIFORM_BUFFER_BINDINGS),
            max_vertex_blocks: get_integer(gl::MAX_VERTEX_UNIFORM_BLOCKS),
            max_geometry_blocks: get_integer(gl::MAX_GEOMETRY_UNIFORM_BLOCKS),
            max_fragment_blocks: get_integer(gl::MAX_FRAGMENT_UNIFORM_BLOCKS),
            max_block_size: get_integer(gl::MAX_UNIFORM_BLOCK_SIZE),
            offset_alignment: get_integer(gl::UNIFORM_BUFFER_OFFSET_ALIGNMENT)
        }
    }
}

fn get_integer(property: GLenum) -> GLint {
    unsafe {
        let mut value = 0;
        gl::GetIntegerv(property, &mut value);
        check_error!();
        value
    }
}