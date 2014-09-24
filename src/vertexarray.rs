
use gl;
use gl::types::{GLint,GLuint,GLboolean,GLsizei,GLvoid};

use super::Context;
use super::Bind;
use super::util::check_error;

use super::IndexBufferHandle;
use super::VertexBufferHandle;

#[deriving(Clone)]
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

// pub type QuickVertexAttribute (u8, AttributeType, bool)

struct VertexArrayLifetime {
    pub id: u32
}

impl VertexArrayLifetime {
    fn new() -> VertexArrayLifetime {
        let mut id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            check_error();
        }
        VertexArrayLifetime { id: id }
    }
}

impl Drop for VertexArrayLifetime {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
            check_error();
        }
    }
}

pub struct VertexArray {
    lifetime: VertexArrayLifetime,
    vertex_attributes: Vec<VertexAttribute>,
    index_buffer: Option<IndexBufferHandle>
}

impl VertexArray {
    pub fn new(ctx: &mut Context,
               attributes: &[VertexAttribute],
               index_buffer: Option<IndexBufferHandle>) -> VertexArray {
        let vertex_array = VertexArray {
            lifetime: VertexArrayLifetime::new(),
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

    /* pub fn new_single_vbo(attributes: &[QuickVertexAttribute],
            vertex_buffer: VertexBufferHandle,
            index_buffer: Option<IndexBufferHandle>) -> VertexArray {
    } */

    fn set_vertex_attribute(ctx: &mut Context, attribute: &VertexAttribute) {
        ctx.bind_vbo_for_editing(&attribute.vertex_buffer);
        let attribute_type = match attribute.attribute_type {
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
        };
        unsafe {
            gl::VertexAttribPointer(
                attribute.index as GLuint,
                attribute.size as GLint,
                attribute_type,
                attribute.normalized as GLboolean,
                attribute.stride as GLsizei,
                attribute.offset as *const GLvoid
                );
            check_error();
        }
    }
}

impl Bind for VertexArray {
    fn bind(&self) {
        gl::BindVertexArray(self.lifetime.id);
    }

    fn get_id(&self) -> u32 {
        self.lifetime.id
    }
}