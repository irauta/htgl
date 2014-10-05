
use gl;
use gl::types::{GLint,GLsizei,GLvoid,GLenum};

use super::{Context,VertexArrayHandle,ProgramHandle};
use super::options::{mod,RenderOption};

pub enum PrimitiveMode {
    Triangles
}

pub struct Renderer<'a> {
    context: &'a mut Context
}

impl<'a> Renderer<'a> {
    pub fn new(context: &'a mut Context) -> Renderer<'a> {
        Renderer { context: context }
    }

    pub fn use_vertex_array(&mut self, vao: &VertexArrayHandle) {
        self.context.vao_tracker.bind(vao.access());
    }

    pub fn use_program(&mut self, program: &ProgramHandle) {
        self.context.program_tracker.bind(program.access());
    }

    pub fn draw_arrays(&self, primitive_mode: PrimitiveMode, first: u32, count: u32) {
        let primitive_mode = gl_primitive_mode(primitive_mode);
        gl::DrawArrays(primitive_mode, first as GLint, count as GLsizei);
        check_error!();
    }

    pub fn draw_elements_u8(&self, primitive_mode: PrimitiveMode, count: u32, start: u32) {
        let primitive_mode = gl_primitive_mode(primitive_mode);
        self.draw_elements(primitive_mode, count, gl::UNSIGNED_BYTE, start);
    }

    pub fn draw_elements_u16(&self, primitive_mode: PrimitiveMode, count: u32, start: u32) {
        let primitive_mode = gl_primitive_mode(primitive_mode);
        self.draw_elements(primitive_mode, count, gl::UNSIGNED_SHORT, start);
    }

    pub fn draw_elements_u32(&self, primitive_mode: PrimitiveMode, count: u32, start: u32) {
        let primitive_mode = gl_primitive_mode(primitive_mode);
        self.draw_elements(primitive_mode, count, gl::UNSIGNED_INT, start);
    }

    fn draw_elements(&self, primitive_mode: GLenum, count: u32, index_type: GLenum, start: u32) {
        unsafe {
            let start = start as *const GLvoid;
            gl::DrawElements(primitive_mode, count as GLint, index_type, start);
            check_error!();
        }
    }

    pub fn clear(&mut self) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        check_error!();
    }

    pub fn set_option(&mut self, option: RenderOption) {
        options::set_option(option);
    }
}

fn gl_primitive_mode(primitive_mode: PrimitiveMode) -> GLenum {
    match primitive_mode {
        Triangles => gl::TRIANGLES
    }
}