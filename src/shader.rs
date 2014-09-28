
use std::str::{Slice,Owned};

use gl;
use gl::types::{GLenum,GLint,GLsizei};

use super::ShaderHandle;
use super::util::check_error;

pub enum ShaderType {
    VertexShader,
    FragmentShader
}

pub struct Shader {
    id: u32
}

impl Shader {
    pub fn new(shader_type: ShaderType, source: &str) -> Shader {
        let id = gl::CreateShader(shader_type_to_enum(shader_type));
        check_error();
        let shader = Shader { id: id };
        shader.compile(source);
        shader
    }

    pub fn get_info_log(&self) -> String {
        let info_length = self.get_info_length();
        let mut actual_info_length = 0;
        let mut info_vec = Vec::with_capacity(info_length as uint);
        info_vec.grow(info_length as uint, 0u8);
        unsafe {
            let info_vec_ptr = info_vec.as_mut_ptr() as *mut i8;
            gl::GetShaderInfoLog(self.id, info_length, &mut actual_info_length, info_vec_ptr);
            check_error();
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
            check_error();

            gl::CompileShader(self.id);
            check_error();
            self.get_compile_status();
        }
    }

    fn get_compile_status(&self) -> bool {
        let mut compile_status = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut compile_status);
            check_error();
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
            check_error();
        }
        info_length
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::DeleteShader(self.id);
        check_error();
    }
}

pub struct ProgramLifetime {
    id: u32
}

impl ProgramLifetime {
    fn new() -> ProgramLifetime {
        let id = gl::CreateProgram();
        check_error();
        ProgramLifetime { id: id }
    }
}

impl Drop for ProgramLifetime {
    fn drop(&mut self) {
        gl::DeleteProgram(self.id);
        check_error();
    }
}

pub struct Program {
    lifetime: ProgramLifetime,
    shaders: Vec<ShaderHandle>
}

impl Program {
    pub fn new(shaders: &[ShaderHandle]) -> Program {
        let program = Program {
            lifetime: ProgramLifetime::new(),
            shaders: shaders.to_vec()
        };
        for ref shader in program.shaders.iter() {
            gl::AttachShader(program.lifetime.id, shader.access().id);
            check_error();
        }
        gl::LinkProgram(program.lifetime.id);
        check_error();
        program
    }

    pub fn use_program(&self) {
        gl::UseProgram(self.lifetime.id);
    }

    pub fn get_info_log() -> String {
        fail!();
    }
}

fn shader_type_to_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        VertexShader => gl::VERTEX_SHADER,
        FragmentShader => gl::FRAGMENT_SHADER
    }
}