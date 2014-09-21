
extern crate gl;

use std::rc::Rc;

pub use context::Context;
pub use gl::load_with;

mod context;
mod buffer;
mod util;
mod tracker;


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

pub type VertexBufferHandle = Handle<buffer::BufferObject<buffer::VertexBuffer>>;
pub type IndexBufferHandle = Handle<buffer::BufferObject<buffer::IndexBuffer>>;
