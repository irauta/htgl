
use super::Bind;
use super::vertexarray::VertexArray;

pub struct SimpleBindingTracker<T> {
    currently_bound: u32
}

impl<T: Bind> SimpleBindingTracker<T> {
    pub fn new() -> SimpleBindingTracker<T> {
        SimpleBindingTracker { currently_bound: 0 }
    }

    pub fn bind(&mut self, resource: &T) {
        let id = resource.get_id();
        if self.currently_bound != id {
            resource.bind();
            self.currently_bound = id;
        }
    }
}

pub struct VertexArrayTracker {
    currently_bound: u32
}

impl VertexArrayTracker {
    pub fn new() -> VertexArrayTracker {
        VertexArrayTracker { currently_bound: 0 }
    }

    pub fn bind_for_editing(&mut self, vertex_array: &VertexArray) {
        let id = vertex_array.get_id();
        if self.currently_bound != id {
            vertex_array.bind();
            self.currently_bound = id;
        }
    }
}