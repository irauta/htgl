
use gl;

use super::super::context::{Context,RegistrationHandle,ContextEditingSupport};
use super::super::tracker::TrackerId;
use super::BufferObject;
use super::super::vertexarray::VertexArray;

pub struct IndexBufferTag;

pub type IndexBuffer = BufferObject<IndexBufferTag>;

pub fn new_index_buffer(tracker_id: TrackerId, registration: RegistrationHandle) -> IndexBuffer {
    BufferObject::new(tracker_id, gl::ELEMENT_ARRAY_BUFFER, registration)
}

pub fn new_index_buffer_editor<'a>(context: &'a mut Context, vertex_array: &'a VertexArray) -> IndexBufferEditor<'a> {
    context.bind_vao_for_editing(vertex_array);
    IndexBufferEditor { context: context, vertex_array: vertex_array }
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

    pub fn sub_data_u8(&mut self, data: &[u8], byte_offset: usize) {
        self.sub_data(data, byte_offset);
    }

    pub fn sub_data_u16(&mut self, data: &[u16], byte_offset: usize) {
        self.sub_data(data, byte_offset);
    }

    pub fn sub_data_u32(&mut self, data: &[u32], byte_offset: usize) {
        self.sub_data(data, byte_offset);
    }

    fn data<D>(&mut self, data: &[D]) {
        if let Some(ref index_buffer) = self.vertex_array.index_buffer() {
            index_buffer.data(data);
        }
    }

    fn sub_data<D>(&mut self, data: &[D], byte_offset: usize) {
        if let Some(ref index_buffer) = self.vertex_array.index_buffer() {
            index_buffer.sub_data(data, byte_offset);
        }
    }
}
