
use gl;

use super::super::context::{Context,RegistrationHandle,ContextEditingSupport};
use super::super::tracker::TrackerId;
use super::BufferObject;

pub struct VertexBufferTag;

pub type VertexBuffer = BufferObject<VertexBufferTag>;

pub fn new_vertex_buffer(tracker_id: TrackerId, registration: RegistrationHandle) -> VertexBuffer {
    BufferObject::new(tracker_id, gl::ARRAY_BUFFER, registration)
}

pub fn new_vertex_buffer_editor<'a>(context: &'a mut Context, vertex_buffer: &'a VertexBuffer) -> VertexBufferEditor<'a> {
    context.bind_vbo_for_editing(vertex_buffer);
    VertexBufferEditor { context: context, vertex_buffer: vertex_buffer }
}

pub struct VertexBufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    vertex_buffer: &'a VertexBuffer
}

impl<'a> VertexBufferEditor<'a> {
    pub fn data<D>(&mut self, data: &[D]) {
        self.vertex_buffer.data(data);
    }

    pub fn sub_data<D>(&mut self, data: &[D], byte_offset: usize) {
        self.vertex_buffer.sub_data(data, byte_offset);
    }
}
