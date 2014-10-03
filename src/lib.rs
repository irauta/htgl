
#![feature(unsafe_destructor,macro_rules,slicing_syntax,if_let)]

extern crate core;
extern crate gl;

pub use gl::load_with;
pub use vertexarray::{VertexAttribute,AttributeType,AttributeByte,AttributeUnsignedByte,AttributeShort,AttributeUnsignedShort,AttributeInt,AttributeUnsignedInt,AttributeHalfFloat,AttributeFloat,AttributeDouble,AttributeInt2101010Rev,AttributeUnsignedInt2101010Rev};
pub use shader::{ShaderType,VertexShader,FragmentShader};
pub use options::{RenderOption,ClearColor,DepthTest};
pub use draw::Drawer;
pub use editor::{VertexBufferEditor,IndexBufferEditor};

use core::cell::RefCell;
use std::rc::Rc;
use buffer::VertexBuffer;
use vertexarray::VertexArray;
use context::{SharedContextState,RegistrationHandle};

macro_rules! check_error(
    () => (::util::check_error(file!(), line!()));
)

mod buffer;
mod util;
mod tracker;
mod vertexarray;
mod shader;
mod options;
mod draw;
mod context;
mod editor;

pub type VertexBufferHandle = Handle<buffer::VertexBuffer>;
pub type IndexBufferHandle = Handle<buffer::IndexBuffer>;
pub type VertexArrayHandle = Handle<vertexarray::VertexArray>;
pub type ShaderHandle = Handle<shader::Shader>;
pub type ProgramHandle = Handle<shader::Program>;


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
    shared_state: Rc<RefCell<SharedContextState>>
}

impl Context {
    pub fn new() -> Context {
        Context { shared_state: Rc::new(RefCell::new(SharedContextState::new())) }
    }

    // Construct new objects

    pub fn new_vertex_buffer(&mut self) -> VertexBufferHandle {
        let registration = self.registration_handle();
        Handle::new(buffer::new_vertex_buffer(registration))
    }

    pub fn new_index_buffer(&mut self) -> IndexBufferHandle {
        let registration = self.registration_handle();
        Handle::new(buffer::new_index_buffer(registration))
    }

    pub fn new_vertex_array(&mut self,
                            attributes: &[VertexAttribute],
                            index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let registration = self.registration_handle();
        Handle::new(vertexarray::VertexArray::new(self, attributes, index_buffer, registration))
    }

    pub fn new_vertex_array_simple(&mut self,
                                   attributes: &[(u8, AttributeType, bool)],
                                   vertex_buffer: VertexBufferHandle,
                                   index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let registration = self.registration_handle();
        Handle::new(vertexarray::VertexArray::new_single_vbo(self, attributes, vertex_buffer, index_buffer, registration))
    }

    pub fn new_shader(&self, shader_type: ShaderType, source: &str) -> ShaderHandle {
        let registration = self.registration_handle();
        Handle::new(shader::Shader::new(shader_type, source, registration))
    }

    pub fn new_program(&self, shaders: &[ShaderHandle]) -> ProgramHandle {
        let registration = self.registration_handle();
        Handle::new(shader::Program::new(shaders, registration))
    }

    // Modify object contents with the help of editor objects

    pub fn edit_vertex_buffer<'a>(&'a mut self, vbo: &'a VertexBufferHandle) -> VertexBufferEditor {
        VertexBufferEditor::new(self, vbo.access())
    }

    pub fn edit_index_buffer<'a>(&'a mut self, vao: &'a VertexArrayHandle) -> Option<IndexBufferEditor> {
        let vao = vao.access();
        match vao.index_buffer() {
            Some(_) => Some(IndexBufferEditor::new(self, vao)),
            None => None
        }
    }

    // Commands that do not (directly) consume resources

    pub fn drawer<'a>(&'a mut self) -> Drawer {
        Drawer::new(self)
    }

    pub fn clear(&mut self) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        check_error!();
    }

    pub fn set_option(&mut self, option: RenderOption) {
        options::set_option(option);
    }

    // Internal stuff

    fn bind_vbo(&mut self, vbo: &VertexBufferHandle) {
        let vbo = vbo.access();
        self.shared_state.borrow_mut().vbo_tracker.bind(vbo);
    }

    fn bind_vao(&mut self, vao: &VertexArray) {
        self.shared_state.borrow_mut().vao_tracker.bind(vao);
    }

    fn registration_handle(&self) -> RegistrationHandle {
        RegistrationHandle::new(self.shared_state.clone())
    }
}

#[unsafe_destructor]
impl Drop for Context {
    fn drop(&mut self) {
        self.shared_state.borrow_mut().is_alive = false;
    }
}