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

use gl;
use gl::types::{GLint,GLsizei,GLvoid,GLenum};

use super::{VertexArrayHandle,ProgramHandle};
use super::context::{Context,ContextRenderingSupport};
use super::options::{self,RenderOption};

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
        self.context.bind_vao_for_rendering(vao);
    }

    pub fn use_program(&mut self, program: &ProgramHandle) {
        self.context.bind_program_for_rendering(program);
    }

    pub fn draw_arrays(&mut self, primitive_mode: PrimitiveMode, first: u32, count: u32) {
        let primitive_mode = gl_primitive_mode(primitive_mode);
        self.context.prepare_for_rendering();
        unsafe {
            gl::DrawArrays(primitive_mode, first as GLint, count as GLsizei);
        }
        check_error!();
    }

    pub fn draw_elements_u8(&mut self, primitive_mode: PrimitiveMode, count: u32, start: u32) {
        let primitive_mode = gl_primitive_mode(primitive_mode);
        self.draw_elements(primitive_mode, count, gl::UNSIGNED_BYTE, start);
    }

    pub fn draw_elements_u16(&mut self, primitive_mode: PrimitiveMode, count: u32, start: u32) {
        let primitive_mode = gl_primitive_mode(primitive_mode);
        self.draw_elements(primitive_mode, count, gl::UNSIGNED_SHORT, start);
    }

    pub fn draw_elements_u32(&mut self, primitive_mode: PrimitiveMode, count: u32, start: u32) {
        let primitive_mode = gl_primitive_mode(primitive_mode);
        self.draw_elements(primitive_mode, count, gl::UNSIGNED_INT, start);
    }

    fn draw_elements(&mut self, primitive_mode: GLenum, count: u32, index_type: GLenum, start: u32) {
        self.context.prepare_for_rendering();
        unsafe {
            let start = start as *const GLvoid;
            gl::DrawElements(primitive_mode, count as GLint, index_type, start);
            check_error!();
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        check_error!();
    }

    pub fn set_option(&mut self, option: RenderOption) {
        options::set_option(option);
    }
}

fn gl_primitive_mode(primitive_mode: PrimitiveMode) -> GLenum {
    match primitive_mode {
        PrimitiveMode::Triangles => gl::TRIANGLES
    }
}