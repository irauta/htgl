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

//! This module handles shaders. Shaders are individual parts of the programmable rendering
//! pipeline, handling a single task within it. For example, there are vertex and fragment
//! shaders, each handling vertex manipulation and fragment color generation. You should consult
//! OpenGL documentation on the intricacies of shaders and programs in OpenGL.
//!
//! The basic idea is that you compile individual shaders, then link them into a program. A shader
//! may be used in many programs.

use std::iter::repeat;

use gl;
use gl::types::{GLenum,GLint,GLsizei};

use super::util::vec_to_string;
use super::context::RegistrationHandle;

/// Supported shader types.
pub enum ShaderType {
    VertexShader,
    FragmentShader
}

/// A shader object. It can be created, it's info log can be queried and it can be linked into a
/// program.
pub struct Shader {
    id: u32,
    registration: RegistrationHandle,
}

impl Shader {
    /// Create and compile a shader from the given source. See glCreateShader, glShaderSource and 
    /// glCompileShader.
    pub fn new(shader_type: ShaderType, source: &str, registration: RegistrationHandle) -> Shader {
        let id = unsafe { gl::CreateShader(shader_type_to_enum(shader_type)) };
        check_error!();
        let shader = Shader { id: id, registration: registration };
        shader.compile(source);
        shader
    }

    /// Identify the shader. The returned value is the actual OpenGL object name.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_info_log(&self) -> String {
        let info_length = self.get_info_length();
        let mut actual_info_length = 0;
        let mut info_vec: Vec<u8> = repeat(0u8).take(info_length as usize).collect();
        unsafe {
            let info_vec_ptr = info_vec.as_mut_ptr() as *mut i8;
            gl::GetShaderInfoLog(self.id, info_length, &mut actual_info_length, info_vec_ptr);
            check_error!();
        }
        info_vec.pop(); // Remove the null byte from end
        vec_to_string(info_vec)
    }

    fn compile(&self, source: &str) {
        unsafe {
            let length = source.len() as GLint;
            let source_ptr = source.as_ptr() as *const i8;
            let source_ptr_ptr = &source_ptr as *const *const i8;
            gl::ShaderSource(self.id, 1, source_ptr_ptr, &length);
            check_error!();

            gl::CompileShader(self.id);
            check_error!();
        }
    }

    fn get_compile_status(&self) -> bool {
        let mut compile_status = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut compile_status);
            check_error!();
        }
        compile_status == (gl::TRUE as i32)
    }

    fn get_info_length(&self) -> GLsizei {
        let mut info_length = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut info_length);
            check_error!();
        }
        info_length
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if self.registration.context_alive() {
            unsafe {
                gl::DeleteShader(self.id)
            };
            check_error!();
        }
    }
}

/// This struct enables access to compilation status and info log of a shader.
pub struct ShaderInfoAccessor<'a> {
    shader: &'a Shader
}

impl<'a> ShaderInfoAccessor<'a> {
    /// Returns the shader info log. It may contain useful information about the shader, especially
    /// in the case of error.
    pub fn get_info_log(&self) -> String {
        self.shader.get_info_log()
    }

    /// A simple boolean flag that tells if compiling the shader succeeded or not.
    pub fn get_compile_status(&self) -> bool {
        self.shader.get_compile_status()
    }
}

/// Non-public constructor for the info accessor.
pub fn new_shader_info_accessor(shader: &Shader) -> ShaderInfoAccessor {
    ShaderInfoAccessor { shader: shader }
}

fn shader_type_to_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => gl::VERTEX_SHADER,
        ShaderType::FragmentShader => gl::FRAGMENT_SHADER
    }
}
