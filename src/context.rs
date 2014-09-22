
use super::tracker::SimpleBindingTracker;
use super::buffer::{mod,VertexBuffer};
use super::{Handle,VertexBufferHandle,IndexBufferHandle};

pub struct Context {
    vbo_tracker: SimpleBindingTracker<VertexBuffer>
}

impl Context {
    pub fn new() -> Context {
        Context { vbo_tracker: SimpleBindingTracker::new() }
    }

    pub fn new_vertex_buffer(&self) -> VertexBufferHandle {
        Handle::new(buffer::new_vertex_buffer())
    }

    pub fn new_index_buffer(&self) -> IndexBufferHandle {
        Handle::new(buffer::new_index_buffer())
    }

    pub fn vertex_data<T>(&mut self, vbo: &VertexBufferHandle, data: &[T]) {
        let vbo = vbo.access();
        self.vbo_tracker.bind(vbo);
        vbo.data(data);
    }

    pub fn vertex_sub_data<T>(&mut self, vbo: &VertexBufferHandle, data: &[T], offset: uint) {
        let vbo = vbo.access();
        self.vbo_tracker.bind(vbo);
        vbo.sub_data(data, offset);
    }
}