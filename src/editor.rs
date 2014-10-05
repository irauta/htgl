
use super::Context;
use super::buffer::VertexBuffer;
use super::vertexarray::VertexArray;

pub struct VertexBufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    vertex_buffer: &'a VertexBuffer
}

impl<'a> VertexBufferEditor<'a> {
    pub fn data<D>(&mut self, data: &[D]) {
        self.context.vbo_tracker.bind(self.vertex_buffer);
        self.vertex_buffer.data(data);
    }

    pub fn sub_data<D>(&mut self, data: &[D], byte_offset: uint) {
        self.context.vbo_tracker.bind(self.vertex_buffer);
        self.vertex_buffer.sub_data(data, byte_offset);
    }
}

pub fn new_vertex_buffer_editor<'a>(context: &'a mut Context, vertex_buffer: &'a VertexBuffer) -> VertexBufferEditor<'a> {
    VertexBufferEditor { context: context, vertex_buffer: vertex_buffer }
}


pub struct IndexBufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    vertex_array: &'a VertexArray
}

impl<'a> IndexBufferEditor<'a> {
    pub fn data_u8(&mut self, data: &[u8]) {
        self.data(data);
    }

    pub fn data_u16(&mut self, data: &[u16]) {
        self.data(data);
    }

    pub fn data_u32(&mut self, data: &[u32]) {
        self.data(data);
    }

    pub fn sub_data_u8(&mut self, data: &[u8], byte_offset: uint) {
        self.sub_data(data, byte_offset);
    }

    pub fn sub_data_u16(&mut self, data: &[u16], byte_offset: uint) {
        self.sub_data(data, byte_offset);
    }

    pub fn sub_data_u32(&mut self, data: &[u32], byte_offset: uint) {
        self.sub_data(data, byte_offset);
    }

    fn data<D>(&mut self, data: &[D]) {
        self.context.vao_tracker.bind(self.vertex_array);
        if let Some(ref index_buffer) = self.vertex_array.index_buffer() {
            index_buffer.data(data);
        }
    }

    fn sub_data<D>(&mut self, data: &[D], byte_offset: uint) {
        self.context.vao_tracker.bind(self.vertex_array);
        if let Some(ref index_buffer) = self.vertex_array.index_buffer() {
            index_buffer.sub_data(data, byte_offset);
        }
    }
}

pub fn new_index_buffer_editor<'a>(context: &'a mut Context, vertex_array: &'a VertexArray) -> IndexBufferEditor<'a> {
    IndexBufferEditor { context: context, vertex_array: vertex_array }
}