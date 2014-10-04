
use gl;
use gl::types::{GLint,GLsizei};

use super::{Context,VertexArrayHandle,ProgramHandle};
use super::options::{mod,RenderOption};

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

    pub fn draw_arrays(&self, first: u32, count: u32) {
        gl::DrawArrays(gl::TRIANGLES, first as GLint, count as GLsizei);
        check_error!();
    }

    pub fn clear(&mut self) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        check_error!();
    }

    pub fn set_option(&mut self, option: RenderOption) {
        options::set_option(option);
    }
}