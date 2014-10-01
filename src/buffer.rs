
use gl;
use gl::types::{GLenum,GLsizeiptr,GLvoid};

use std::mem::size_of;

use super::Bind;
use super::SharedContextStateHandle;

pub struct VertexBufferTag;
pub struct IndexBufferTag;

pub type VertexBuffer = BufferObject<VertexBufferTag>;
pub type IndexBuffer = BufferObject<IndexBufferTag>;


pub struct BufferObject<T> {
    pub id: u32,
    context_shared: SharedContextStateHandle,
    target: GLenum
}

impl<T> BufferObject<T> {
    fn new(target: GLenum, context_shared: SharedContextStateHandle) -> BufferObject<T> {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            check_error!();
        }
        BufferObject { id: id, context_shared: context_shared, target: target }
    }

    pub fn data<D>(&self, data: &[D]) {
        let data_size = (size_of::<D>() * data.len()) as GLsizeiptr;
        unsafe {
            gl::BufferData(self.target, data_size, data.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            check_error!();
        }
    }

    pub fn sub_data<D>(&self, data: &[D], offset: uint) {
        let data_size = (size_of::<D>() * data.len()) as GLsizeiptr;
        unsafe {
            gl::BufferSubData(self.target, data_size, offset as GLsizeiptr, data.as_ptr() as *const GLvoid);
            check_error!();
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for BufferObject<T> {
    fn drop(&mut self) {
        let mut context_shared = self.context_shared.borrow_mut();
        if !context_shared.is_alive {
            return;
        }
        context_shared.unregister_buffer(self.id, self.target);
        unsafe {
            gl::DeleteBuffers(1, &self.id);
            check_error!();
        }
    }
}

impl<T> PartialEq for BufferObject<T> {
    fn eq(&self, other: &BufferObject<T>) -> bool {
        self.id == other.id
    }
}

impl<T> Bind for BufferObject<T> {
    fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

pub fn new_vertex_buffer(context_shared: SharedContextStateHandle) -> VertexBuffer {
    BufferObject::new(gl::ARRAY_BUFFER, context_shared)
}

pub fn new_index_buffer(context_shared: SharedContextStateHandle) -> IndexBuffer {
    BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, context_shared)
}