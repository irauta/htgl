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

//! The program module is complex, because it contains also the means to manipulate and query
//! program uniforms and attributes.

use std::iter::repeat;
use std::ffi::CString;

use gl;
use gl::types::GLenum;

use super::util::vec_to_string;
use super::tracker::Bind;
use super::handle::HandleAccess;
use super::context::{Context,RegistrationHandle,ContextEditingSupport};
use super::ShaderHandle;
use super::tracker::TrackerId;

pub use self::uniform::{SimpleUniformTypeFloat,SimpleUniformTypeI32,SimpleUniformTypeMatrix,SimpleUniformTypeU32};
pub use self::uniform::{UniformInfo,Uniform,InterfaceBlock,BlockUniform};
pub use self::attribute::{ShaderAttributeInfo,ShaderAttribute};

mod uniform;
mod attribute;

/// A shader program, formed by linking together `Shader` objects.
pub struct Program {
    id: u32,
    tracker_id: TrackerId,
    registration: RegistrationHandle,
    /// The program keeps the shaders alive even though OpenGL should take care of it. Not sure
    /// at all if really necessary.
    shaders: Vec<ShaderHandle>
}

impl Program {
    /// Create a program, attach shaders to it and link the program.
    pub fn new(tracker_id: TrackerId, shaders: &[ShaderHandle], registration: RegistrationHandle) -> Program {
        let id = unsafe { gl::CreateProgram() };
        check_error!();
        let program = Program {
            id: id,
            tracker_id: tracker_id,
            registration: registration,
            shaders: shaders.to_vec()
        };
        program.link();
        program
    }

    /// See glGetAttribLocation.
    pub fn get_attribute_location(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetAttribLocation(self.id, c_name.as_ptr());
            check_error!();
            location
        }
    }

    /// See glGetUniformLocation.
    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            check_error!();
            location
        }
    }

    /// See glGetFragDataLocation.
    pub fn get_frag_data_location(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetFragDataLocation(self.id, c_name.as_ptr());
            check_error!();
            location
        }
    }

    /// See glGetFragDataIndex.
    pub fn get_frag_data_index(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetFragDataIndex(self.id, c_name.as_ptr());
            check_error!();
            location
        }
    }

    fn link(&self) {
        for ref shader in self.shaders.iter() {
            unsafe {
                gl::AttachShader(self.id, shader.access().get_id());
            }
            check_error!();
        }
        unsafe {
            gl::LinkProgram(self.id);
        }
        check_error!();
    }

    fn get_info_log(&self) -> String {
        let info_length = self.get_value(gl::INFO_LOG_LENGTH);
        let mut actual_info_length = 0;
        let mut info_vec: Vec<u8> = repeat(0u8).take(info_length as usize).collect();
        unsafe {
            let info_vec_ptr = info_vec.as_mut_ptr() as *mut i8;
            gl::GetProgramInfoLog(self.id, info_length, &mut actual_info_length, info_vec_ptr);
            check_error!();
        }
        info_vec.pop(); // Remove the null byte from end
        vec_to_string(info_vec)
    }

    fn get_link_status(&self) -> bool {
        let link_status = self.get_value(gl::LINK_STATUS);
        link_status == (gl::TRUE as i32)
    }

    fn get_value(&self, property: GLenum) -> i32 {
        let mut value = 0;
        unsafe {
            gl::GetProgramiv(self.id, property, &mut value);
            check_error!();
        }
        value
    }
}


#[unsafe_destructor]
impl Drop for Program {
    fn drop(&mut self) {
        if self.registration.context_alive() {
            unsafe {
                gl::DeleteProgram(self.id);
            }
            check_error!();
        }
    }
}

impl Bind for Program {
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn get_id(&self) -> TrackerId {
        self.tracker_id
    }
}

/// Program info accessor provides info on a program.
pub struct ProgramInfoAccessor<'a> {
    program: &'a Program
}

impl<'a> ProgramInfoAccessor<'a> {
    /// See glGetAttribLocation.
    pub fn get_attribute_location(&self, name: &str) -> i32 {
        self.program.get_attribute_location(name)
    }

    /// See glGetUniformLocation.
    pub fn get_uniform_location(&self, name: &str) -> i32 {
        self.program.get_uniform_location(name)
    }

    /// Returns information on all the uniforms of the program in one go, including the blocks.
    /// See `UniformInfo`.
    pub fn get_uniform_info(&self) -> UniformInfo {
        uniform::make_uniform_info(self.program)
    }

    /// Returns information on all the attributes of the program in one go.
    /// See `ShaderAttributeInfo`.
    pub fn get_attribute_info(&self) -> ShaderAttributeInfo {
        attribute::make_attribute_info_vec(self.program)
    }

    /// See glGetFragDataLocation.
    pub fn get_frag_data_location(&self, name: &str) -> i32 {
        self.program.get_frag_data_location(name)
    }

    /// See glGetFragDataIndex.
    pub fn get_frag_data_index(&self, name: &str) -> i32 {
        self.program.get_frag_data_index(name)
    }

    /// Was the program linked successfully?
    pub fn get_link_status(&self) -> bool {
        self.program.get_link_status()
    }

    /// Program info log may contain information relevant to the linking.
    pub fn get_info_log(&self) -> String {
        self.program.get_info_log()
    }
}

/// Constructor not visible to library users.
pub fn new_program_info_accessor(program: &Program) -> ProgramInfoAccessor {
    ProgramInfoAccessor { program: program }
}

/// Program editor allows settings uniform values.
pub struct ProgramEditor<'a> {
    /// Borrow context even though it's not used to prevent other actions on it while editing program.
    #[allow(dead_code)]
    context: &'a mut Context,
    /// Borrow program too for the same reason as the context.
    #[allow(dead_code)]
    program: &'a Program
}

impl<'a> ProgramEditor<'a> {
    /// Specify a uniform value (or multiple values of single uniform) of type f32.
    /// You must specify exactly the right amount of values, for example if count is 1 and
    /// uniform_type is Uniform3f, it is an error for values slice to contain less than 3 values.
    /// If the slice is longer than needed, the extra values are ignored.
    /// This method will panic if the minimum number of values is not given to it!
    /// For OpenGL documentation, see glUniform*fv.
    pub fn uniform_f32(&self, location: i32, count: usize, uniform_type: SimpleUniformTypeFloat, values: &[f32]) {
        uniform::uniform_f32(location, count, uniform_type, values)
    }

    /// Specify a matrix uniform value.
    /// See notes on the uniform_f32 for correct use - giving too few values will cause a panic!
    /// For OpenGL documentation, see glUniformMatrix*fv.
    pub fn uniform_matrix(&self, location: i32, count: usize, uniform_type: SimpleUniformTypeMatrix, transpose: bool, values: &[f32]) {
        uniform::uniform_matrix(location, count, uniform_type, transpose, values)
    }

    /// Specify a uniform value (or multiple values of single uniform) of type u32.
    /// See notes on the uniform_f32 for correct use - giving too few values will cause a panic!
    /// For OpenGL documentation, see glUniform*uiv.
    pub fn uniform_u32(&self, location: i32, count: usize, uniform_type: SimpleUniformTypeU32, values: &[u32]) {
        uniform::uniform_u32(location, count, uniform_type, values)
    }

    /// Specify a uniform value (or multiple values of single uniform) of type i32.
    /// See notes on the uniform_f32 for correct use - giving too few values will cause a panic!
    /// For OpenGL documentation, see glUniform*iv.
    pub fn uniform_i32(&self, location: i32, count: usize, uniform_type: SimpleUniformTypeI32, values: &[i32]) {
        uniform::uniform_i32(location, count, uniform_type, values)
    }

    /// Allow accessing program info even during editing the said program. Just a convenience
    /// method not different from the one in `Context`.
    pub fn program_info(&self) -> ProgramInfoAccessor {
        new_program_info_accessor(self.program)
    }
}

/// Non-public constructor for the program editor.
pub fn new_program_editor<'a>(context: &'a mut Context, program: &'a Program) -> ProgramEditor<'a> {
    context.bind_program_for_editing(program);
    ProgramEditor { context: context, program: program }
}