
use super::Context;
use super::buffer::VertexBuffer;
use super::vertexarray::VertexArray;

pub struct VertexBufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    vertex_buffer: &'a VertexBuffer
}

impl<'a> VertexBufferEditor<'a> {
    pub fn new(context: &'a mut Context, vertex_buffer: &'a VertexBuffer) -> VertexBufferEditor<'a> {
        VertexBufferEditor { context: context, vertex_buffer: vertex_buffer }
    }

    pub fn data<D>(&mut self, data: &[D]) {
        self.context.vbo_tracker.bind(self.vertex_buffer);
        self.vertex_buffer.data(data);
    }

    pub fn sub_data<D>(&mut self, data: &[D], offset: uint) {
        self.context.vbo_tracker.bind(self.vertex_buffer);
        self.vertex_buffer.sub_data(data, offset);
    }
}


pub struct IndexBufferEditor<'a> {
    #[allow(dead_code)]
    context: &'a mut Context,
    vertex_array: &'a VertexArray
}

impl<'a> IndexBufferEditor<'a> {
    pub fn new(context: &'a mut Context, vertex_array: &'a VertexArray) -> IndexBufferEditor<'a> {
        IndexBufferEditor { context: context, vertex_array: vertex_array }
    }

    pub fn data<D>(&mut self, data: &[D]) {
        self.context.vao_tracker.bind(self.vertex_array);
        if let Some(ref index_buffer) = self.vertex_array.index_buffer() {
            index_buffer.data(data);
        }
    }

    pub fn sub_data<D>(&mut self, data: &[D], offset: uint) {
        self.context.vao_tracker.bind(self.vertex_array);
        if let Some(ref index_buffer) = self.vertex_array.index_buffer() {
            index_buffer.sub_data(data, offset);
        }
    }
}
