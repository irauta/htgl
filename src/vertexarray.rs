
use gl;
use gl::types::{GLenum,GLint,GLuint,GLboolean,GLsizei,GLvoid};

use super::Context;
use super::Bind;

use super::SharedContextStateHandle;
use super::IndexBufferHandle;
use super::VertexBufferHandle;

#[deriving(Clone,Show)]
pub enum AttributeType {
    AttributeByte,
    AttributeUnsignedByte,
    AttributeShort,
    AttributeUnsignedShort,
    AttributeInt,
    AttributeUnsignedInt,
    AttributeHalfFloat,
    AttributeFloat,
    AttributeDouble,
    AttributeInt2101010Rev,
    AttributeUnsignedInt2101010Rev
}

#[deriving(Clone)]
pub struct VertexAttribute {
    pub index: u32,
    pub size: u8,
    pub attribute_type: AttributeType,
    pub normalized: bool,
    pub stride: u32,
    pub offset: u32,
    pub vertex_buffer: VertexBufferHandle
}

pub struct VertexArray {
    pub id: u32,
    context_shared: SharedContextStateHandle,
    vertex_attributes: Vec<VertexAttribute>,
    index_buffer: Option<IndexBufferHandle>
}

impl VertexArray {
    pub fn new(ctx: &mut Context,
               attributes: &[VertexAttribute],
               index_buffer: Option<IndexBufferHandle>,
               context_shared: SharedContextStateHandle) -> VertexArray {
        let mut id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            check_error!();
        }
        let vertex_array = VertexArray {
            id: id,
            context_shared: context_shared,
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
                          attributes: &[(u8, AttributeType, bool)],
                          vertex_buffer: VertexBufferHandle,
                          index_buffer: Option<IndexBufferHandle>,
                          context_shared: SharedContextStateHandle) -> VertexArray {
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
        VertexArray::new(ctx, full_attributes.as_slice(), index_buffer, context_shared)
    }

    fn set_vertex_attribute(ctx: &mut Context, attribute: &VertexAttribute) {
        ctx.bind_vbo_for_editing(&attribute.vertex_buffer);
        let attribute_type = attribute_to_gl_type(attribute.attribute_type);

        gl::EnableVertexAttribArray(attribute.index);
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

    pub fn bind_vao_by_id(id: u32) {
        gl::BindVertexArray(id);
        check_error!();
    }
}

#[unsafe_destructor]
impl Drop for VertexArray {
    fn drop(&mut self) {
        let mut context_shared = self.context_shared.borrow_mut();
        if !context_shared.is_alive {
            return;
        }
        context_shared.unregister_vertex_array(self.id);
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
            check_error!();
        }
    }
}

impl Bind for VertexArray {
    fn bind(&self) {
        VertexArray::bind_vao_by_id(self.id);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

fn attribute_to_gl_type(attribute_type: AttributeType) -> GLenum {
    match attribute_type {
        AttributeByte => gl::BYTE,
        AttributeUnsignedByte => gl::UNSIGNED_BYTE,
        AttributeShort => gl::SHORT,
        AttributeUnsignedShort => gl::UNSIGNED_SHORT,
        AttributeInt => gl::INT,
        AttributeUnsignedInt => gl::UNSIGNED_INT,
        AttributeHalfFloat => gl::HALF_FLOAT,
        AttributeFloat => gl::FLOAT,
        AttributeDouble => gl::DOUBLE,
        AttributeInt2101010Rev => gl::INT_2_10_10_10_REV,
        AttributeUnsignedInt2101010Rev => gl::UNSIGNED_INT_2_10_10_10_REV
    }
}

fn attribute_to_size(attribute_type: AttributeType) -> GLenum {
    match attribute_type {
        AttributeByte => 1,
        AttributeUnsignedByte => 1,
        AttributeShort => 2,
        AttributeUnsignedShort => 2,
        AttributeInt => 4,
        AttributeUnsignedInt => 4,
        AttributeHalfFloat => 2,
        AttributeFloat => 4,
        AttributeDouble => 8,
        AttributeInt2101010Rev => 4,
        AttributeUnsignedInt2101010Rev => 4
    }
}