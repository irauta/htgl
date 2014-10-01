
use core::cell::RefCell;
use std::rc::Rc;

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

pub struct RegistrationHandle {
    context_shared: Rc<RefCell<SharedContextState>>
}

impl RegistrationHandle {
    pub fn new(context_shared: Rc<RefCell<SharedContextState>>) -> RegistrationHandle {
        RegistrationHandle { context_shared: context_shared }
    }

    pub fn context_alive(&self) -> bool {
        self.context_shared.borrow().is_alive
    }

    pub fn unregister_vertex_array(&self, id: u32) {
        self.context_shared.borrow_mut().unregister_vertex_array(id);
    }

    pub fn unregister_buffer(&self, id: u32, target: GLenum) {
        self.context_shared.borrow_mut().unregister_buffer(id, target);
    }
}