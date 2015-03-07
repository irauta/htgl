
use gl;

use super::super::context::{Context,RegistrationHandle,ContextEditingSupport};
use super::super::tracker::TrackerId;
use super::BufferObject;

pub struct UniformBufferTag;

pub type UniformBuffer = BufferObject<UniformBufferTag>;

pub fn new_uniform_buffer(tracker_id: TrackerId, registration: RegistrationHandle) -> UniformBuffer {
    BufferObject::new(tracker_id, gl::UNIFORM_BUFFER, registration)
}

pub fn new_uniform_buffer_editor<'a>(context: &'a mut Context, uniform_buffer: &'a UniformBuffer) -> UniformBufferEditor<'a> {
    context.bind_ubo_for_editing(uniform_buffer);
    UniformBufferEditor { context: context, uniform_buffer: uniform_buffer }
}

pub struct UniformBufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    uniform_buffer: &'a UniformBuffer
}

impl<'a> UniformBufferEditor<'a> {
    pub fn data<D>(&mut self, data: &[D]) {
        self.uniform_buffer.data(data);
    }

    pub fn sub_data<D>(&mut self, data: &[D], byte_offset: usize) {
        self.uniform_buffer.sub_data(data, byte_offset);
    }
}
