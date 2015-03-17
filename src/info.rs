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

use gl;
use gl::types::{GLint,GLenum};

#[derive(Debug)]
pub struct ContextInfo {
    pub uniform_buffer: UniformBufferInfo
}

#[derive(Debug)]
pub struct UniformBufferInfo {
    pub max_bindings: GLint,
    pub max_geometry_blocks: GLint,
    pub max_vertex_blocks: GLint,
    pub max_fragment_blocks: GLint,
    pub max_block_size: GLint,
    pub offset_alignment: GLint
}

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