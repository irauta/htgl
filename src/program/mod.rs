
use gl;
use gl::types::GLenum;

use super::util::vec_to_string;
use super::tracker::Bind;
use super::handle::HandleAccess;
use super::context::{Context,RegistrationHandle,ContextEditingSupport};
use super::ShaderHandle;
use super::tracker::TrackerId;
use super::{SimpleUniformTypeFloat,SimpleUniformTypeInt,SimpleUniformTypeMatrix,SimpleUniformTypeUint};

use self::uniform::UniformInfo;
use self::attribute::ShaderAttributeInfo;

mod uniform;
mod attribute;

pub struct Program {
    id: u32,
    tracker_id: TrackerId,
    registration: RegistrationHandle,
    shaders: Vec<ShaderHandle>
}

impl Program {
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

    pub fn get_attribute_location(&self, name: &str) -> i32 {
        let c_name = name.to_c_str();
        unsafe {
            let location = gl::GetAttribLocation(self.id, c_name.as_ptr());
            check_error!();
            location
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let c_name = name.to_c_str();
        unsafe {
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            check_error!();
            location
        }
    }

    pub fn get_frag_data_location(&self, name: &str) -> i32 {
        let c_name = name.to_c_str();
        unsafe {
            let location = gl::GetFragDataLocation(self.id, c_name.as_ptr());
            check_error!();
            location
        }
    }

    pub fn get_frag_data_index(&self, name: &str) -> i32 {
        let c_name = name.to_c_str();
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

pub struct ProgramInfoAccessor<'a> {
    program: &'a Program
}

impl<'a> ProgramInfoAccessor<'a> {
    pub fn get_attribute_location(&self, name: &str) -> i32 {
        self.program.get_attribute_location(name)
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        self.program.get_uniform_location(name)
    }

    pub fn get_uniform_info(&self) -> UniformInfo {
        uniform::make_uniform_info(self.program)
    }

    pub fn get_attribute_info(&self) -> ShaderAttributeInfo {
        attribute::make_attribute_info_vec(self.program)
    }

    pub fn get_frag_data_location(&self, name: &str) -> i32 {
        self.program.get_frag_data_location(name)
    }

    pub fn get_frag_data_index(&self, name: &str) -> i32 {
        self.program.get_frag_data_index(name)
    }

    pub fn get_link_status(&self) -> bool {
        self.program.get_link_status()
    }

    pub fn get_info_log(&self) -> String {
        self.program.get_info_log()
    }
}

pub fn new_program_info_accessor(program: &Program) -> ProgramInfoAccessor {
    ProgramInfoAccessor { program: program }
}

pub struct ProgramEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    #[allow(dead_code)]
    program: &'a Program
}

impl<'a> ProgramEditor<'a> {
    pub fn uniform_f32(&self, location: i32, count: uint, uniform_type: SimpleUniformTypeFloat, values: &[f32]) {
        uniform::uniform_f32(location, count, uniform_type, values)
    }

    pub fn uniform_matrix(&self, location: i32, count: uint, uniform_type: SimpleUniformTypeMatrix, transpose: bool, values: &[f32]) {
        uniform::uniform_matrix(location, count, uniform_type, transpose, values)
    }

    pub fn uniform_u32(&self, location: i32, count: uint, uniform_type: SimpleUniformTypeUint, values: &[u32]) {
        uniform::uniform_u32(location, count, uniform_type, values)
    }

    pub fn uniform_i32(&self, location: i32, count: uint, uniform_type: SimpleUniformTypeInt, values: &[i32]) {
        uniform::uniform_i32(location, count, uniform_type, values)
    }

    pub fn program_info(&self) -> ProgramInfoAccessor {
        new_program_info_accessor(self.program)
    }
}

pub fn new_program_editor<'a>(context: &'a mut Context, program: &'a Program) -> ProgramEditor<'a> {
    context.bind_program_for_editing(program);
    ProgramEditor { context: context, program: program }
}