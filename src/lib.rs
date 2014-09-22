
extern crate gl;

use std::rc::Rc;

pub use context::Context;
pub use gl::load_with;

mod context;
mod buffer;
mod util;
mod tracker;
mod vertexarray;


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

pub type VertexBufferHandle = Handle<buffer::VertexBuffer>;
pub type IndexBufferHandle = Handle<buffer::IndexBuffer>;
pub type VertexArrayHandle = Handle<vertexarray::VertexArray>;