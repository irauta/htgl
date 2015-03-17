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
