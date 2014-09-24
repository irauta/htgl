
extern crate gl;

pub use gl::load_with;
pub use vertexarray::{VertexAttribute,AttributeType};


mod buffer;
mod util;
mod tracker;
mod vertexarray;

use std::rc::Rc;
use tracker::{SimpleBindingTracker,VertexArrayTracker};
use buffer::VertexBuffer;
use vertexarray::VertexArray;


pub type VertexBufferHandle = Handle<buffer::VertexBuffer>;
pub type IndexBufferHandle = Handle<buffer::IndexBuffer>;
pub type VertexArrayHandle = Handle<vertexarray::VertexArray>;


trait Bind {
    fn bind(&self);
    fn get_id(&self) -> u32;
}


pub struct Handle<T> {
    resource: Rc<T>
}

impl<T> Handle<T> {
    fn new(resource: T) -> Handle<T> {
        Handle { resource: Rc::new(resource) }
    }

    fn access(&self) -> &T {
        &*self.resource
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Handle<T> {
        Handle { resource: self.resource.clone() }
    }
}


pub struct Context {
    vbo_tracker: SimpleBindingTracker<VertexBuffer>,
    vao_tracker: VertexArrayTracker
}

impl Context {
    pub fn new() -> Context {
        Context { vbo_tracker: SimpleBindingTracker::new(), vao_tracker: VertexArrayTracker::new() }
    }

    pub fn new_vertex_buffer(&self) -> VertexBufferHandle {
        Handle::new(buffer::new_vertex_buffer())
    }

    pub fn new_index_buffer(&self) -> IndexBufferHandle {
        Handle::new(buffer::new_index_buffer())
    }

    pub fn new_vertex_array(&mut self,
                            attributes: &[VertexAttribute],
                            index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        Handle::new(vertexarray::VertexArray::new(self, attributes, index_buffer))
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

    fn bind_vbo_for_editing(&mut self, vbo: &VertexBufferHandle) {
        let vbo = vbo.access();
        self.vbo_tracker.bind(vbo);
    }

    fn bind_vao_for_editing(&mut self, vao: &VertexArray) {
        self.vao_tracker.bind_for_editing(vao);
    }
}