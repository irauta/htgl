
use gl;
use gl::types::{GLenum,GLsizeiptr,GLvoid};

use std::mem::size_of;

use super::Bind;
use super::util::check_error;

pub struct VertexBuffer;
pub struct IndexBuffer;

struct BufferLifetime {
    pub id: u32
}

impl BufferLifetime {
    fn new() -> BufferLifetime {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            check_error();
        }
        BufferLifetime { id: id }
    }
}

impl Drop for BufferLifetime {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
            check_error();
        }
    }
}

pub struct BufferObject<T> {
    lifetime: BufferLifetime,
    target: GLenum
}

impl<T> BufferObject<T> {
    fn new(target: GLenum) -> BufferObject<T> {
        BufferObject { lifetime: BufferLifetime::new(), target: target }
    }

    pub fn data<D>(&self, data: &[D]) {
        let data_size = (size_of::<D>() * data.len()) as GLsizeiptr;
        unsafe {
            gl::BufferData(self.target, data_size, data.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            check_error();
        }
    }

    pub fn sub_data<D>(&self, data: &[D], offset: uint) {
        let data_size = (size_of::<D>() * data.len()) as GLsizeiptr;
        unsafe {
            gl::BufferSubData(self.target, data_size, offset as GLsizeiptr, data.as_ptr() as *const GLvoid);
            check_error();
        }
    }
}

impl<T> PartialEq for BufferObject<T> {
    fn eq(&self, other: &BufferObject<T>) -> bool {
        self.lifetime.id == other.lifetime.id
    }
}

impl<T> Bind for BufferObject<T> {
    fn bind(&self) {
        gl::BindBuffer(self.target, self.lifetime.id);
    }

    fn get_id(&self) -> u32 {
        self.lifetime.id
    }
}

pub fn new_vertex_buffer() -> BufferObject<VertexBuffer> {
    BufferObject::new(gl::ARRAY_BUFFER)
}

pub fn new_index_buffer() -> BufferObject<IndexBuffer> {
    BufferObject::new(gl::ELEMENT_ARRAY_BUFFER)
}