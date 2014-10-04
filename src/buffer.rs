
use gl;
use gl::types::{GLenum,GLsizeiptr,GLvoid};

use std::mem::size_of;

use super::Bind;
use super::context::RegistrationHandle;
use super::tracker::TrackerId;

pub struct VertexBufferTag;
pub struct IndexBufferTag;

pub type VertexBuffer = BufferObject<VertexBufferTag>;
pub type IndexBuffer = BufferObject<IndexBufferTag>;


pub struct BufferObject<T> {
    pub id: u32,
    tracker_id: TrackerId,
    registration: RegistrationHandle,
    target: GLenum
}

impl<T> BufferObject<T> {
    fn new(tracker_id: TrackerId, target: GLenum, registration: RegistrationHandle) -> BufferObject<T> {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            check_error!();
        }
        BufferObject { id: id, tracker_id: tracker_id, registration: registration, target: target }
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
        if self.registration.context_alive() {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
                check_error!();
            }
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

    fn get_id(&self) -> TrackerId {
        self.tracker_id
    }
}

pub fn new_vertex_buffer(tracker_id: TrackerId, registration: RegistrationHandle) -> VertexBuffer {
    BufferObject::new(tracker_id, gl::ARRAY_BUFFER, registration)
}

pub fn new_index_buffer(tracker_id: TrackerId, registration: RegistrationHandle) -> IndexBuffer {
    BufferObject::new(tracker_id, gl::ELEMENT_ARRAY_BUFFER, registration)
}