
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
use super::VertexAttributeType;

#[deriving(Clone)]
pub struct VertexAttribute {
    pub index: u32,
    pub size: u8,
    pub attribute_type: VertexAttributeType,
    pub normalized: bool,
    pub stride: u32,
    pub offset: u32,
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
        VertexArray::new(ctx, tracker_id, full_attributes.as_slice(), index_buffer, registration)
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
        super::VertexAttributeByte => gl::BYTE,
        super::VertexAttributeUnsignedByte => gl::UNSIGNED_BYTE,
        super::VertexAttributeShort => gl::SHORT,
        super::VertexAttributeUnsignedShort => gl::UNSIGNED_SHORT,
        super::VertexAttributeInt => gl::INT,
        super::VertexAttributeUnsignedInt => gl::UNSIGNED_INT,
        super::VertexAttributeHalfFloat => gl::HALF_FLOAT,
        super::VertexAttributeFloat => gl::FLOAT,
        super::VertexAttributeDouble => gl::DOUBLE,
        super::VertexAttributeInt2101010Rev => gl::INT_2_10_10_10_REV,
        super::VertexAttributeUnsignedInt2101010Rev => gl::UNSIGNED_INT_2_10_10_10_REV
    }
}

fn attribute_to_size(attribute_type: VertexAttributeType) -> GLenum {
    match attribute_type {
        super::VertexAttributeByte => 1,
        super::VertexAttributeUnsignedByte => 1,
        super::VertexAttributeShort => 2,
        super::VertexAttributeUnsignedShort => 2,
        super::VertexAttributeInt => 4,
        super::VertexAttributeUnsignedInt => 4,
        super::VertexAttributeHalfFloat => 2,
        super::VertexAttributeFloat => 4,
        super::VertexAttributeDouble => 8,
        super::VertexAttributeInt2101010Rev => 4,
        super::VertexAttributeUnsignedInt2101010Rev => 4
    }
}