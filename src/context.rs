
use gl;
use gl::types::GLenum;

use super::tracker::{SimpleBindingTracker,VertexArrayTracker};
use super::buffer::VertexBuffer;

pub struct SharedContextState {
    pub is_alive: bool,
    pub vbo_tracker: SimpleBindingTracker<VertexBuffer>,
    pub vao_tracker: VertexArrayTracker
}

impl SharedContextState {
    pub fn new() -> SharedContextState {
        SharedContextState {
            is_alive: true,
            vbo_tracker: SimpleBindingTracker::new(),
            vao_tracker: VertexArrayTracker::new()
        }
    }

    pub fn unregister_vertex_array(&mut self, id: u32) {
        self.vao_tracker.unregister(id);
    }

    pub fn unregister_buffer(&mut self, id: u32, target: GLenum) {
        match target {
            gl::ARRAY_BUFFER => self.vbo_tracker.unregister(id),
            gl::ELEMENT_ARRAY_BUFFER => {}, // VAO tracker handles index buffers implicitly
            _ => fail!("unregister_buffer not implemented for target {}", target)
        }
    }
}