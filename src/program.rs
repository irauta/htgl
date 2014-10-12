
use std::fmt::Show;

use gl;
use gl::types::GLenum;

use super::util::vec_to_string;
use super::tracker::Bind;
use super::handle::HandleAccess;
use super::context::{Context,RegistrationHandle,ContextEditingSupport};
use super::ShaderHandle;
use super::tracker::TrackerId;
use super::{UniformTypeFloat,UniformTypeInt,UniformTypeMatrix,UniformTypeUint};

pub struct Program {
    id: u32,
    tracker_id: TrackerId,
    registration: RegistrationHandle,
    shaders: Vec<ShaderHandle>
}

impl Program {
    pub fn new(tracker_id: TrackerId, shaders: &[ShaderHandle], registration: RegistrationHandle) -> Program {
        let id = gl::CreateProgram();
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

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let c_name = name.to_c_str();
        unsafe {
            gl::GetUniformLocation(self.id, c_name.as_ptr())
        }
    }

    pub fn get_uniform_block_index(&self, name: &str) -> u32 {
        let c_name = name.to_c_str();
        unsafe {
            gl::GetUniformBlockIndex(self.id, c_name.as_ptr())
        }
    }

    pub fn get_active_uniforms(&self) -> Vec<(i32, String)> {
        let mut uniforms = Vec::new();
        let count = self.get_value(gl::ACTIVE_UNIFORMS) as u32;
        if count == 0 {
            return uniforms;
        }
        let max_length = self.get_value(gl::ACTIVE_UNIFORM_MAX_LENGTH);
        for index in range(0, count) {
            let mut name_vec = Vec::with_capacity(max_length as uint);
            name_vec.grow(max_length as uint, 0u8);
            let mut length = 0;
            let mut size = 0;
            let mut uniform_type = 0;
            let mut location = 0;
            unsafe {
                let name_ptr = name_vec.as_mut_ptr() as *mut i8;
                gl::GetActiveUniform(self.id, index, max_length, &mut length, &mut size, &mut uniform_type, name_ptr);
                if length == 0 {
                    continue;
                }
                name_vec.truncate(length as uint);
                let name_ptr = name_vec.as_ptr() as *const i8;
                location = gl::GetUniformLocation(self.id, name_ptr);
            }
            uniforms.push((location, vec_to_string(name_vec)));
        }
        uniforms
    }

    fn link(&self) {
        for ref shader in self.shaders.iter() {
            gl::AttachShader(self.id, shader.access().get_id());
            check_error!();
        }
        gl::LinkProgram(self.id);
        check_error!();
        self.get_link_status();
    }

    fn get_info_log(&self) -> String {
        let info_length = self.get_value(gl::INFO_LOG_LENGTH);
        let mut actual_info_length = 0;
        let mut info_vec = Vec::with_capacity(info_length as uint);
        info_vec.grow(info_length as uint, 0u8);
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
        let link_status = link_status != (gl::FALSE as i32);
        if !link_status {
            println!("Program info log:\n{}", self.get_info_log());
            fail!("Compiling failed");
        }
        link_status
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
            gl::DeleteProgram(self.id);
            check_error!();
        }
    }
}

impl Bind for Program {
    fn bind(&self) {
        gl::UseProgram(self.id);
    }

    fn get_id(&self) -> TrackerId {
        self.tracker_id
    }
}

pub struct ProgramEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    program: &'a Program
}

impl<'a> ProgramEditor<'a> {
    pub fn get_uniform_location(&self, name: &str) -> i32 {
        self.program.get_uniform_location(name)
    }

    pub fn get_uniform_block_index(&self, name: &str) -> u32 {
        self.program.get_uniform_block_index(name)
    }

    pub fn get_active_uniforms(&self) -> Vec<(i32, String)> {
        self.program.get_active_uniforms()
    }

    pub fn uniform_f32(&self, location: i32, count: uint, uniform_type: UniformTypeFloat, values: &[f32]) {
        validate_uniform_f32(count, uniform_type, values);
        let count = count as i32;
        unsafe {
            let value_ptr = values.as_ptr();
            match uniform_type {
                super::Uniform1f => gl::Uniform1fv(location, count, value_ptr),
                super::Uniform2f => gl::Uniform2fv(location, count, value_ptr),
                super::Uniform3f => gl::Uniform3fv(location, count, value_ptr),
                super::Uniform4f => gl::Uniform4fv(location, count, value_ptr)
            }
        }
    }

    pub fn uniform_matrix(&self, location: i32, count: uint, uniform_type: UniformTypeMatrix, transpose: bool, values: &[f32]) {
        validate_uniform_matrix(count, uniform_type, values);
        let count = count as i32;
        let transpose = if transpose { gl::TRUE } else { gl::FALSE };
        unsafe {
            let value_ptr = values.as_ptr();
            match uniform_type {
                super::UniformMatrix2f => gl::UniformMatrix2fv(location, count, transpose, value_ptr),
                super::UniformMatrix3f => gl::UniformMatrix3fv(location, count, transpose, value_ptr),
                super::UniformMatrix4f => gl::UniformMatrix4fv(location, count, transpose, value_ptr),
                super::UniformMatrix2x3f => gl::UniformMatrix2x3fv(location, count, transpose, value_ptr),
                super::UniformMatrix3x2f => gl::UniformMatrix3x2fv(location, count, transpose, value_ptr),
                super::UniformMatrix2x4f => gl::UniformMatrix2x4fv(location, count, transpose, value_ptr),
                super::UniformMatrix4x2f => gl::UniformMatrix4x2fv(location, count, transpose, value_ptr),
                super::UniformMatrix3x4f => gl::UniformMatrix3x4fv(location, count, transpose, value_ptr),
                super::UniformMatrix4x3f => gl::UniformMatrix4x3fv(location, count, transpose, value_ptr),
            }
        }
    }

    pub fn uniform_u32(&self, location: i32, count: uint, uniform_type: UniformTypeUint, values: &[u32]) {
        validate_uniform_u32(count, uniform_type, values);
        let count = count as i32;
        unsafe {
            let value_ptr = values.as_ptr();
            match uniform_type {
                super::Uniform1u => gl::Uniform1uiv(location, count, value_ptr),
                super::Uniform2u => gl::Uniform2uiv(location, count, value_ptr),
                super::Uniform3u => gl::Uniform3uiv(location, count, value_ptr),
                super::Uniform4u => gl::Uniform4uiv(location, count, value_ptr),
            }
        }
    }

    pub fn uniform_i32(&self, location: i32, count: uint, uniform_type: UniformTypeInt, values: &[i32]) {
        validate_uniform_i32(count, uniform_type, values);
        let count = count as i32;
        unsafe {
            let value_ptr = values.as_ptr();
            match uniform_type {
                super::Uniform1i => gl::Uniform1iv(location, count, value_ptr),
                super::Uniform2i => gl::Uniform2iv(location, count, value_ptr),
                super::Uniform3i => gl::Uniform3iv(location, count, value_ptr),
                super::Uniform4i => gl::Uniform4iv(location, count, value_ptr),
            }
        }
    }
}

fn validate_uniform_f32(count: uint, uniform_type: UniformTypeFloat, values: &[f32]) {
    let element_count = match uniform_type {
        super::Uniform1f => 1,
        super::Uniform2f => 2,
        super::Uniform3f => 3,
        super::Uniform4f => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_matrix(count: uint, uniform_type: UniformTypeMatrix, values: &[f32]) {
    let element_count = match uniform_type {
        super::UniformMatrix2f => 2 * 2,
        super::UniformMatrix3f => 3 * 3,
        super::UniformMatrix4f => 4 * 4,
        super::UniformMatrix2x3f => 2 * 3,
        super::UniformMatrix3x2f => 3 * 2,
        super::UniformMatrix2x4f => 2 * 4,
        super::UniformMatrix4x2f => 4 * 2,
        super::UniformMatrix3x4f => 3 * 4,
        super::UniformMatrix4x3f => 4 * 3
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_u32(count: uint, uniform_type: UniformTypeUint, values: &[u32]) {
    let element_count = match uniform_type {
        super::Uniform1u => 1,
        super::Uniform2u => 2,
        super::Uniform3u => 3,
        super::Uniform4u => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_i32(count: uint, uniform_type: UniformTypeInt, values: &[i32]) {
    let element_count = match uniform_type {
        super::Uniform1i => 1,
        super::Uniform2i => 2,
        super::Uniform3i => 3,
        super::Uniform4i => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform<T, U: Show>(count: uint, uniform_type: U, element_count: uint, values: &[T]) {
    let expected_len = count * element_count;
    if expected_len > values.len() {
        fail!("Too small uniform value slice: {} of {} would take {} elements, but only got {}",
            count, uniform_type, expected_len, values.len());
    }
}

pub fn new_program_editor<'a>(context: &'a mut Context, program: &'a Program) -> ProgramEditor<'a> {
    context.bind_program_for_editing(program);
    ProgramEditor { context: context, program: program }
}