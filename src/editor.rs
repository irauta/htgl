
use std::fmt::Show;

use gl;

use super::Context;
use super::buffer::VertexBuffer;
use super::vertexarray::VertexArray;
use super::shader::Program;

pub struct VertexBufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    vertex_buffer: &'a VertexBuffer
}

impl<'a> VertexBufferEditor<'a> {
    pub fn data<D>(&mut self, data: &[D]) {
        self.vertex_buffer.data(data);
    }

    pub fn sub_data<D>(&mut self, data: &[D], byte_offset: uint) {
        self.vertex_buffer.sub_data(data, byte_offset);
    }
}

pub fn new_vertex_buffer_editor<'a>(context: &'a mut Context, vertex_buffer: &'a VertexBuffer) -> VertexBufferEditor<'a> {
    context.bind_vbo_for_editing(vertex_buffer);
    VertexBufferEditor { context: context, vertex_buffer: vertex_buffer }
}


pub struct IndexBufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    vertex_array: &'a VertexArray
}

impl<'a> IndexBufferEditor<'a> {
    pub fn data_u8(&mut self, data: &[u8]) {
        self.data(data);
    }

    pub fn data_u16(&mut self, data: &[u16]) {
        self.data(data);
    }

    pub fn data_u32(&mut self, data: &[u32]) {
        self.data(data);
    }

    pub fn sub_data_u8(&mut self, data: &[u8], byte_offset: uint) {
        self.sub_data(data, byte_offset);
    }

    pub fn sub_data_u16(&mut self, data: &[u16], byte_offset: uint) {
        self.sub_data(data, byte_offset);
    }

    pub fn sub_data_u32(&mut self, data: &[u32], byte_offset: uint) {
        self.sub_data(data, byte_offset);
    }

    fn data<D>(&mut self, data: &[D]) {
        if let Some(ref index_buffer) = self.vertex_array.index_buffer() {
            index_buffer.data(data);
        }
    }

    fn sub_data<D>(&mut self, data: &[D], byte_offset: uint) {
        if let Some(ref index_buffer) = self.vertex_array.index_buffer() {
            index_buffer.sub_data(data, byte_offset);
        }
    }
}

pub fn new_index_buffer_editor<'a>(context: &'a mut Context, vertex_array: &'a VertexArray) -> IndexBufferEditor<'a> {
    context.bind_vao_for_editing(vertex_array);
    IndexBufferEditor { context: context, vertex_array: vertex_array }
}

#[deriving(Show)]
pub enum UniformTypeFloat {
    Uniform1f,
    Uniform2f,
    Uniform3f,
    Uniform4f
}

#[deriving(Show)]
pub enum UniformTypeMatrix {
    UniformMatrix2f,
    UniformMatrix3f,
    UniformMatrix4f,
    UniformMatrix2x3f,
    UniformMatrix3x2f,
    UniformMatrix2x4f,
    UniformMatrix4x2f,
    UniformMatrix3x4f,
    UniformMatrix4x3f
}

#[deriving(Show)]
pub enum UniformTypeInt {
    Uniform1i,
    Uniform2i,
    Uniform3i,
    Uniform4i
}

#[deriving(Show)]
pub enum UniformTypeUint {
    Uniform1u,
    Uniform2u,
    Uniform3u,
    Uniform4u
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
                Uniform1f => gl::Uniform1fv(location, count, value_ptr),
                Uniform2f => gl::Uniform2fv(location, count, value_ptr),
                Uniform3f => gl::Uniform3fv(location, count, value_ptr),
                Uniform4f => gl::Uniform4fv(location, count, value_ptr)
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
                UniformMatrix2f => gl::UniformMatrix2fv(location, count, transpose, value_ptr),
                UniformMatrix3f => gl::UniformMatrix3fv(location, count, transpose, value_ptr),
                UniformMatrix4f => gl::UniformMatrix4fv(location, count, transpose, value_ptr),
                UniformMatrix2x3f => gl::UniformMatrix2x3fv(location, count, transpose, value_ptr),
                UniformMatrix3x2f => gl::UniformMatrix3x2fv(location, count, transpose, value_ptr),
                UniformMatrix2x4f => gl::UniformMatrix2x4fv(location, count, transpose, value_ptr),
                UniformMatrix4x2f => gl::UniformMatrix4x2fv(location, count, transpose, value_ptr),
                UniformMatrix3x4f => gl::UniformMatrix3x4fv(location, count, transpose, value_ptr),
                UniformMatrix4x3f => gl::UniformMatrix4x3fv(location, count, transpose, value_ptr),
            }
        }
    }

    pub fn uniform_u32(&self, location: i32, count: uint, uniform_type: UniformTypeUint, values: &[u32]) {
        validate_uniform_u32(count, uniform_type, values);
        let count = count as i32;
        unsafe {
            let value_ptr = values.as_ptr();
            match uniform_type {
                Uniform1u => gl::Uniform1uiv(location, count, value_ptr),
                Uniform2u => gl::Uniform2uiv(location, count, value_ptr),
                Uniform3u => gl::Uniform3uiv(location, count, value_ptr),
                Uniform4u => gl::Uniform4uiv(location, count, value_ptr),
            }
        }
    }

    pub fn uniform_i32(&self, location: i32, count: uint, uniform_type: UniformTypeInt, values: &[i32]) {
        validate_uniform_i32(count, uniform_type, values);
        let count = count as i32;
        unsafe {
            let value_ptr = values.as_ptr();
            match uniform_type {
                Uniform1i => gl::Uniform1iv(location, count, value_ptr),
                Uniform2i => gl::Uniform2iv(location, count, value_ptr),
                Uniform3i => gl::Uniform3iv(location, count, value_ptr),
                Uniform4i => gl::Uniform4iv(location, count, value_ptr),
            }
        }
    }
}

fn validate_uniform_f32(count: uint, uniform_type: UniformTypeFloat, values: &[f32]) {
    let element_count = match uniform_type {
        Uniform1f => 1,
        Uniform2f => 2,
        Uniform3f => 3,
        Uniform4f => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_matrix(count: uint, uniform_type: UniformTypeMatrix, values: &[f32]) {
    let element_count = match uniform_type {
        UniformMatrix2f => 2 * 2,
        UniformMatrix3f => 3 * 3,
        UniformMatrix4f => 4 * 4,
        UniformMatrix2x3f => 2 * 3,
        UniformMatrix3x2f => 3 * 2,
        UniformMatrix2x4f => 2 * 4,
        UniformMatrix4x2f => 4 * 2,
        UniformMatrix3x4f => 3 * 4,
        UniformMatrix4x3f => 4 * 3
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_u32(count: uint, uniform_type: UniformTypeUint, values: &[u32]) {
    let element_count = match uniform_type {
        Uniform1u => 1,
        Uniform2u => 2,
        Uniform3u => 3,
        Uniform4u => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_i32(count: uint, uniform_type: UniformTypeInt, values: &[i32]) {
    let element_count = match uniform_type {
        Uniform1i => 1,
        Uniform2i => 2,
        Uniform3i => 3,
        Uniform4i => 4
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