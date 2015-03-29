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

//! Vertex arrays are a fairly important concept in OpenGL 3+ core. They encapsulate the state that
//! describes the vertex attributes of one or more vertex buffer used at the same time to render
//! something. Vertex array also contains the index buffer binding state - it is not possible to
//! have an index buffer bound without having a vertex array object bound.
//! See `VertexArray`.

use gl;
use gl::types::{GLenum,GLint,GLuint,GLboolean,GLsizei,GLvoid};

use super::Context;
use super::tracker::Bind;

use super::context::{RegistrationHandle,ContextEditingSupport};
use super::handle::HandleAccess;
use super::IndexBufferHandle;
use super::VertexBufferHandle;
use super::buffer::indexbuffer::IndexBuffer;
use super::tracker::TrackerId;

/// Vertex attribute types, meaning the data type of a single attribute.
#[derive(Copy,Clone,Debug)]
pub enum VertexAttributeType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Int2101010Rev,
    UnsignedInt2101010Rev
}

/// Vertex arrays are meta data objects containing info of several vertex attributes. This struct
/// describes a single attribute. For information on specifics of it, see glVertexAttribPointer.
#[derive(Clone)]
pub struct VertexAttribute {
    pub index: u32,
    pub size: u8,
    pub attribute_type: VertexAttributeType,
    pub normalized: bool,
    pub stride: u32,
    pub offset: u32,
    /// This is not an explicit parameter of glVertexAttribPointer. In the raw OpenGL API, the
    /// vertex buffer bound at the moment of calling glVertexAttribPointer is taken to be part
    /// of the vertex array state. Here it is given explicitly.
    pub vertex_buffer: VertexBufferHandle
}

pub struct VertexArray {
    pub id: u32,
    tracker_id: TrackerId,
    registration: RegistrationHandle,
    vertex_attributes: Vec<VertexAttribute>,
    index_buffer: Option<IndexBufferHandle>
}

impl VertexArray {
    /// Create a vertex array, the longer format.
    pub fn new(ctx: &mut Context,
               tracker_id: TrackerId,
               attributes: &[VertexAttribute],
               index_buffer: Option<IndexBufferHandle>,
               registration: RegistrationHandle) -> VertexArray {
        let mut id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            check_error!();
        }
        let vertex_array = VertexArray {
            id: id,
            tracker_id: tracker_id,
            registration: registration,
            vertex_attributes: attributes.to_vec(),
            index_buffer: index_buffer
        };
        ctx.bind_vao_for_editing(&vertex_array);
        for attribute in vertex_array.vertex_attributes.iter() {
            VertexArray::set_vertex_attribute(ctx, attribute);
        }
        match vertex_array.index_buffer {
            Some(ref index_buffer) => index_buffer.access().bind(),
            None => {}
        }
        vertex_array
    }

    /// Create a vertex array, the simple format (only use a single vertex buffer for all
    /// attributes)
    pub fn new_single_vbo(ctx: &mut Context,
                          tracker_id: TrackerId,
                          attributes: &[(u8, VertexAttributeType, bool)],
                          vertex_buffer: VertexBufferHandle,
                          index_buffer: Option<IndexBufferHandle>,
                          registration: RegistrationHandle) -> VertexArray {
        let mut full_attributes = Vec::with_capacity(attributes.len());
        let mut counter = 0;
        let mut offset = 0;
        for attr in attributes.iter() {
            let (size, attribute_type, normalized) = *attr;
            full_attributes.push(VertexAttribute {
                index: counter,
                size: size,
                attribute_type: attribute_type,
                normalized: normalized,
                stride: 0,
                offset: offset,
                vertex_buffer: vertex_buffer.clone()
            });
            counter += 1;
            offset += attribute_to_size(attribute_type) * size as u32;
        }
        let stride = offset;
        for ref mut attr in full_attributes.iter_mut() {
            attr.stride = stride;
        }
        VertexArray::new(ctx, tracker_id, &full_attributes[..], index_buffer, registration)
    }

    fn set_vertex_attribute(ctx: &mut Context, attribute: &VertexAttribute) {
        ctx.bind_vbo_for_editing(attribute.vertex_buffer.access());
        let attribute_type = attribute_to_gl_type(attribute.attribute_type);

        unsafe {
            gl::EnableVertexAttribArray(attribute.index);
        }
        check_error!();
        unsafe {
            gl::VertexAttribPointer(
                attribute.index as GLuint,
                attribute.size as GLint,
                attribute_type,
                attribute.normalized as GLboolean,
                attribute.stride as GLsizei,
                attribute.offset as *const GLvoid
                );
            check_error!();
        }
    }

    /// What is the index buffer bound to the vertex array, if any.
    pub fn index_buffer<'a>(&'a self) -> Option<&'a IndexBuffer> {
        match self.index_buffer {
            Some(ref handle) => Some(handle.access()),
            None => None
        }
    }
}

#[unsafe_destructor]
impl Drop for VertexArray {
    fn drop(&mut self) {
        if self.registration.context_alive() {
            unsafe {
                gl::DeleteVertexArrays(1, &self.id);
                check_error!();
            }
        }
    }
}

impl Bind for VertexArray {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        check_error!();
    }

    fn get_id(&self) -> TrackerId {
        self.tracker_id
    }
}

fn attribute_to_gl_type(attribute_type: VertexAttributeType) -> GLenum {
    match attribute_type {
        VertexAttributeType::Byte => gl::BYTE,
        VertexAttributeType::UnsignedByte => gl::UNSIGNED_BYTE,
        VertexAttributeType::Short => gl::SHORT,
        VertexAttributeType::UnsignedShort => gl::UNSIGNED_SHORT,
        VertexAttributeType::Int => gl::INT,
        VertexAttributeType::UnsignedInt => gl::UNSIGNED_INT,
        VertexAttributeType::HalfFloat => gl::HALF_FLOAT,
        VertexAttributeType::Float => gl::FLOAT,
        VertexAttributeType::Double => gl::DOUBLE,
        VertexAttributeType::Int2101010Rev => gl::INT_2_10_10_10_REV,
        VertexAttributeType::UnsignedInt2101010Rev => gl::UNSIGNED_INT_2_10_10_10_REV
    }
}

fn attribute_to_size(attribute_type: VertexAttributeType) -> GLenum {
    match attribute_type {
        VertexAttributeType::Byte => 1,
        VertexAttributeType::UnsignedByte => 1,
        VertexAttributeType::Short => 2,
        VertexAttributeType::UnsignedShort => 2,
        VertexAttributeType::Int => 4,
        VertexAttributeType::UnsignedInt => 4,
        VertexAttributeType::HalfFloat => 2,
        VertexAttributeType::Float => 4,
        VertexAttributeType::Double => 8,
        VertexAttributeType::Int2101010Rev => 4,
        VertexAttributeType::UnsignedInt2101010Rev => 4
    }
}