
use std::str::{Slice,Owned};

use gl;
use gl::types::{GLenum,GLint,GLsizei};

use super::Bind;
use super::context::RegistrationHandle;
use super::ShaderHandle;
use super::tracker::TrackerId;

pub enum ShaderType {
    VertexShader,
    FragmentShader
}

pub struct Shader {
    id: u32,
    registration: RegistrationHandle,
}

impl Shader {
    pub fn new(shader_type: ShaderType, source: &str, registration: RegistrationHandle) -> Shader {
        let id = gl::CreateShader(shader_type_to_enum(shader_type));
        check_error!();
        let shader = Shader { id: id, registration: registration };
        shader.compile(source);
        shader
    }

    fn get_info_log(&self) -> String {
        let info_length = self.get_info_length();
        let mut actual_info_length = 0;
        let mut info_vec = Vec::with_capacity(info_length as uint);
        info_vec.grow(info_length as uint, 0u8);
        unsafe {
            let info_vec_ptr = info_vec.as_mut_ptr() as *mut i8;
            gl::GetShaderInfoLog(self.id, info_length, &mut actual_info_length, info_vec_ptr);
            check_error!();
        }
        info_vec.pop(); // Remove the null byte from end
        match String::from_utf8(info_vec) {
            Ok(info) => info,
            Err(info_vec) => match String::from_utf8_lossy(info_vec[]) {
                Owned(info) => info,
                Slice(info_str) => String::from_str(info_str) // This one shouldn't probably happen
            }
        }
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
            self.get_compile_status();
        }
    }

    fn get_compile_status(&self) -> bool {
        let mut compile_status = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut compile_status);
            check_error!();
        }
        let compile_status = compile_status != (gl::FALSE as i32);
        if !compile_status {
            println!("Shader info log:\n{}", self.get_info_log());
            fail!("Compiling failed");
        }
        compile_status
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

#[unsafe_destructor]
impl Drop for Shader {
    fn drop(&mut self) {
        if self.registration.context_alive() {
            gl::DeleteShader(self.id);
            check_error!();
        }
    }
}

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
            gl::AttachShader(self.id, shader.access().id);
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

fn shader_type_to_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        VertexShader => gl::VERTEX_SHADER,
        FragmentShader => gl::FRAGMENT_SHADER
    }
}

/// Always shorten the vector to exclude the null byte!
fn vec_to_string(vec: Vec<u8>) -> String {
    match String::from_utf8(vec) {
        Ok(string) => string,
        Err(vec) => match String::from_utf8_lossy(vec[]) {
            Owned(string) => string,
            Slice(str_slice) => String::from_str(str_slice) // This one shouldn't probably happen
        }
    }
}