
use gl;
use gl::types::{GLint,GLsizei};

use super::{Context,VertexArrayHandle,ProgramHandle};

pub struct Drawer<'a> {
    context: &'a mut Context
}

impl<'a> Drawer<'a> {
    pub fn new(context: &'a mut Context) -> Drawer<'a> {
        Drawer { context: context }
    }

    pub fn use_vertex_array(&mut self, vao: &VertexArrayHandle) {
        self.context.vao_tracker.bind(vao.access());
    }

    pub fn use_program(&mut self, program: &ProgramHandle) {
        self.context.program_tracker.bind(program.access());
    }

    pub fn draw_arrays(&self, first: u32, count: u32) {
        gl::DrawArrays(gl::TRIANGLES, first as GLint, count as GLsizei);
        check_error!();
    }
}