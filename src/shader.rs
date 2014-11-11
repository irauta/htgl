
use gl;
use gl::types::{GLenum,GLint,GLsizei};

use super::util::vec_to_string;
use super::context::RegistrationHandle;
use super::ShaderType;

pub struct Shader {
    id: u32,
    registration: RegistrationHandle,
}

impl Shader {
    pub fn new(shader_type: ShaderType, source: &str, registration: RegistrationHandle) -> Shader {
        let id = unsafe { gl::CreateShader(shader_type_to_enum(shader_type)) };
        check_error!();
        let shader = Shader { id: id, registration: registration };
        shader.compile(source);
        shader
    }

    pub fn get_id(&self) -> u32 {
        self.id
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
            panic!("Compiling shader failed");
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
            unsafe {
                gl::DeleteShader(self.id)
            };
            check_error!();
        }
    }
}

fn shader_type_to_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        super::VertexShader => gl::VERTEX_SHADER,
        super::FragmentShader => gl::FRAGMENT_SHADER
    }
}
