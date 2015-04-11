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

//! Buffers are central to modern OpenGL as a bulk source of data, alongside textures.
//!
//! As with "bare" OpenGL buffers the buffers provided by this library are basically untyped,
//! in the sense that the same buffer can be used as, for example, vertex and index buffer.
//! When editing buffer contents, you must choose to edit it as a buffer of some specific type.
//! See `Context::edit_vertex_buffer`, `Context::edit_uniform_buffer` and the like. The OpenGL
//! driver may take the type information as a hint on how to allocate memory for the buffer, but
//! this is in no way guaranteed. Modern implementations are actually likely to consider all
//! buffers equal, considering that is how the new APIs (Vulkan, D3D12) work.
//!
//! Note that to draw with a vertex buffer and an index buffer, they must be attached to an
//! vertex array object, and then use (read: bind) it to have the buffers in use when drawing.

use gl;
use gl::types::{GLenum,GLsizeiptr,GLvoid};

use std::mem::size_of;

use super::tracker::Bind;
use super::context::{Context,ContextEditingSupport,RegistrationHandle};
use super::vertexarray::VertexArray;
use super::tracker::TrackerId;

/// The different recognized buffer types.
#[derive(Clone,Copy,Debug)]
pub enum BufferType {
    /// GL_ARRAY_BUFFER
    VertexBuffer,
    /// GL_ELEMENT_ARRAY_BUFFER
    IndexBuffer,
    /// GL_UNIFORM_BUFFER
    UniformBuffer
}

fn type_to_target(buffer_type: BufferType) -> GLenum {
    match buffer_type {
        BufferType::VertexBuffer => gl::ARRAY_BUFFER,
        BufferType::IndexBuffer => gl::ELEMENT_ARRAY_BUFFER,
        BufferType::UniformBuffer => gl::UNIFORM_BUFFER
    }
}

/// Buffer object structure.
pub struct BufferObject {
    pub id: u32,
    tracker_id: TrackerId,
    registration: RegistrationHandle
}

/// Create a new buffer object.
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

    /// Bind the buffer. Not really to be used directly!
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

/// Helper type that binds the buffers for binding trackers.
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

/// Bind buffer as VBO and edit it.
pub fn new_vertex_buffer_editor<'a>(context: &'a mut Context, buffer: &'a BufferObject) -> BufferEditor<'a> {
    context.bind_vbo_for_editing(buffer);
    BufferEditor { context: context, buffer: buffer, buffer_type: BufferType::VertexBuffer }
}

/// Bind the vertex array object the IBO is associated with(!) and edit it.
pub fn new_index_buffer_editor<'a>(context: &'a mut Context, vertex_array: &'a VertexArray, buffer: &'a BufferObject) -> BufferEditor<'a> {
    context.bind_vao_for_editing(vertex_array);
    BufferEditor { context: context, buffer: buffer, buffer_type: BufferType::IndexBuffer }
}

/// Bind buffer as UBO and edit it.
pub fn new_uniform_buffer_editor<'a>(context: &'a mut Context, buffer: &'a BufferObject) -> BufferEditor<'a> {
    context.bind_ubo_for_editing(buffer);
    BufferEditor { context: context, buffer: buffer, buffer_type: BufferType::UniformBuffer }
}

/// Buffer editor is used to edit contents of a buffer object of any type.
pub struct BufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    buffer: &'a BufferObject,
    buffer_type: BufferType
}

impl<'a> BufferEditor<'a> {
    /// Replace the data store of the buffer object. This effectively resizes the buffer, but the
    /// old contents are lost.
    ///
    /// See glBufferData.
    pub fn data<D>(&mut self, data: &[D]) {
        self.buffer.data(self.buffer_type, data);
    }

    /// Replace a region of values within the buffer.
    ///
    /// See glBufferSubData.
    pub fn sub_data<D>(&mut self, data: &[D], byte_offset: usize) {
        self.buffer.sub_data(self.buffer_type, data, byte_offset);
    }
}
