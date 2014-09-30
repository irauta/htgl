
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

    pub fn unregister(&mut self, id: u32) {
        if self.currently_bound == id {
            self.currently_bound = 0;
        }
    }
}

pub struct VertexArrayTracker {
    currently_bound: u32,
    bound_for_drawing: u32
}

impl VertexArrayTracker {
    pub fn new() -> VertexArrayTracker {
        VertexArrayTracker { currently_bound: 0, bound_for_drawing: 0 }
    }

    pub fn bind_for_editing(&mut self, vertex_array: &VertexArray) {
        let id = vertex_array.get_id();
        if self.currently_bound != id {
            vertex_array.bind();
            self.currently_bound = id;
        }
    }

    pub fn bind_for_drawing(&mut self, vertex_array: &VertexArray) {
        let id = vertex_array.get_id();
        self.bound_for_drawing = id;
    }

    pub fn prepare_for_drawing(&mut self) {
        let draw_id = self.bound_for_drawing;
        if self.currently_bound != draw_id {
            VertexArray::bind_vao_by_id(draw_id);
            self.currently_bound = draw_id;
        }
    }

    pub fn unregister(&mut self, vao_id: u32) {
        if self.currently_bound == vao_id {
            self.currently_bound = 0;
        }
        if self.bound_for_drawing == vao_id {
            self.bound_for_drawing = 0;
        }
    }
}