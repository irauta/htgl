
use super::IndexBufferHandle;
use super::VertexBufferHandle;

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

pub struct VertexAttribute {
    index: uint,
    size: uint,
    attribute_type: AttributeType,
    normalized: bool,
    stride: uint,
    offset: uint,
    vertex_buffer: VertexBufferHandle
}

// pub type QuickVertexAttribute (u8, AttributeType, bool)

pub struct VertexArray {
    vertex_attributes: Vec<VertexAttribute>,
    index_buffer: Option<IndexBufferHandle>
}

impl VertexArray {
    pub fn new(attributes: &[VertexAttribute], index_buffer: Option<IndexBufferHandle>) -> VertexArray {
        VertexArray { vertex_attributes: Vec::new(), index_buffer: index_buffer }
    }

    /* pub fn new_single_vbo(attributes: &[QuickVertexAttribute],
            vertex_buffer: VertexBufferHandle,
            index_buffer: Option<IndexBufferHandle>) -> VertexArray {
    } */
}