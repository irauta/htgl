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
use gl::types::{GLenum,GLsizeiptr,GLvoid};

use std::mem::size_of;

use super::tracker::Bind;
use super::context::{Context,ContextEditingSupport,RegistrationHandle};
use super::vertexarray::VertexArray;
use super::tracker::TrackerId;

#[derive(Clone,Copy,Debug)]
pub enum BufferType {
    VertexBuffer,
    IndexBuffer,
    UniformBuffer
}

fn type_to_target(buffer_type: BufferType) -> GLenum {
    match buffer_type {
        BufferType::VertexBuffer => gl::ARRAY_BUFFER,
        BufferType::IndexBuffer => gl::ELEMENT_ARRAY_BUFFER,
        BufferType::UniformBuffer => gl::UNIFORM_BUFFER
    }
}

pub struct BufferObject {
    pub id: u32,
    tracker_id: TrackerId,
    registration: RegistrationHandle
}

pub fn new_buffer(tracker_id: TrackerId, registration: RegistrationHandle) -> BufferObject {
    BufferObject::new(tracker_id, registration)
}

impl BufferObject {
    fn new(tracker_id: TrackerId, registration: RegistrationHandle) -> BufferObject {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            check_error!();
        }
        BufferObject {
            id: id,
            tracker_id: tracker_id,
            registration: registration
        }
    }

    pub fn data<D>(&self, buffer_type: BufferType, data: &[D]) {
        let data_size = (size_of::<D>() * data.len()) as GLsizeiptr;
        unsafe {
            gl::BufferData(type_to_target(buffer_type), data_size, data.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            check_error!();
        }
    }

    pub fn sub_data<D>(&self, buffer_type: BufferType, data: &[D], byte_offset: usize) {
        let data_size = (size_of::<D>() * data.len()) as GLsizeiptr;
        unsafe {
            gl::BufferSubData(type_to_target(buffer_type), data_size, byte_offset as GLsizeiptr, data.as_ptr() as *const GLvoid);
            check_error!();
        }
    }

    pub fn bind(&self, buffer_type: BufferType) {
        unsafe {
            gl::BindBuffer(type_to_target(buffer_type), self.id);
            check_error!();
        }
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        if self.registration.context_alive() {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
                check_error!();
            }
        }
    }
}

/* impl PartialEq for BufferObject {
    fn eq(&self, other: &BufferObject) -> bool {
        self.id == other.id
    }
} */

pub struct BufferBinder {
    buffer_type: BufferType
}

impl BufferBinder {
    pub fn new(buffer_type: BufferType) -> BufferBinder {
        BufferBinder { buffer_type: buffer_type }
    }
}

impl Bind<BufferObject> for BufferBinder {
    fn bind(&self, buffer: &BufferObject) {
        buffer.bind(self.buffer_type);
    }

    fn get_id(&self, buffer: &BufferObject) -> TrackerId {
        buffer.tracker_id
    }
}

pub fn new_vertex_buffer_editor<'a>(context: &'a mut Context, buffer: &'a BufferObject) -> BufferEditor<'a> {
    context.bind_vbo_for_editing(buffer);
    BufferEditor { context: context, buffer: buffer, buffer_type: BufferType::VertexBuffer }
}

pub fn new_index_buffer_editor<'a>(context: &'a mut Context, vertex_array: &'a VertexArray, buffer: &'a BufferObject) -> BufferEditor<'a> {
    context.bind_vao_for_editing(vertex_array);
    BufferEditor { context: context, buffer: buffer, buffer_type: BufferType::IndexBuffer }
}

pub fn new_uniform_buffer_editor<'a>(context: &'a mut Context, buffer: &'a BufferObject) -> BufferEditor<'a> {
    context.bind_ubo_for_editing(buffer);
    BufferEditor { context: context, buffer: buffer, buffer_type: BufferType::UniformBuffer }
}

pub struct BufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    buffer: &'a BufferObject,
    buffer_type: BufferType
}

impl<'a> BufferEditor<'a> {
    pub fn data<D>(&mut self, buffer_type: BufferType, data: &[D]) {
        self.buffer.data(buffer_type, data);
    }

    pub fn sub_data<D>(&mut self, buffer_type: BufferType, data: &[D], byte_offset: usize) {
        self.buffer.sub_data(buffer_type, data, byte_offset);
    }
}
