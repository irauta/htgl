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
