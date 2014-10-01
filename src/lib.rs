
#![feature(unsafe_destructor,macro_rules)]

extern crate core;
extern crate gl;

pub use gl::load_with;
pub use vertexarray::{VertexAttribute,AttributeType,AttributeByte,AttributeUnsignedByte,AttributeShort,AttributeUnsignedShort,AttributeInt,AttributeUnsignedInt,AttributeHalfFloat,AttributeFloat,AttributeDouble,AttributeInt2101010Rev,AttributeUnsignedInt2101010Rev};
pub use shader::{ShaderType,VertexShader,FragmentShader};
pub use options::{RenderOption,ClearColor,DepthTest};

use core::cell::RefCell;
use std::rc::Rc;
use buffer::VertexBuffer;
use vertexarray::VertexArray;
use context::SharedContextState;

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

type SharedContextStateHandle = Rc<RefCell<SharedContextState>>;

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
    shared_state: SharedContextStateHandle
}

impl Context {
    pub fn new() -> Context {
        Context { shared_state: Rc::new(RefCell::new(SharedContextState::new())) }
    }

    pub fn new_vertex_buffer(&self) -> VertexBufferHandle {
        let ctx_handle = self.shared_state.clone();
        Handle::new(buffer::new_vertex_buffer(ctx_handle))
    }

    pub fn new_index_buffer(&self) -> IndexBufferHandle {
        let ctx_handle = self.shared_state.clone();
        Handle::new(buffer::new_index_buffer(ctx_handle))
    }

    pub fn new_vertex_array(&mut self,
                            attributes: &[VertexAttribute],
                            index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let ctx_handle = self.shared_state.clone();
        Handle::new(vertexarray::VertexArray::new(self, attributes, index_buffer, ctx_handle))
    }

    pub fn new_vertex_array_simple(&mut self,
                                   attributes: &[(u8, AttributeType, bool)],
                                   vertex_buffer: VertexBufferHandle,
                                   index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let ctx_handle = self.shared_state.clone();
        Handle::new(vertexarray::VertexArray::new_single_vbo(self, attributes, vertex_buffer, index_buffer, ctx_handle))
    }

    pub fn new_shader(&mut self, shader_type: ShaderType, source: &str) -> ShaderHandle {
        let ctx_handle = self.shared_state.clone();
        Handle::new(shader::Shader::new(shader_type, source, ctx_handle))
    }

    pub fn new_program(&mut self, shaders: &[ShaderHandle]) -> ProgramHandle {
        let ctx_handle = self.shared_state.clone();
        Handle::new(shader::Program::new(shaders, ctx_handle))
    }

    pub fn vertex_data<T>(&mut self, vbo: &VertexBufferHandle, data: &[T]) {
        let vbo = vbo.access();
        self.shared_state.borrow_mut().vbo_tracker.bind(vbo);
        vbo.data(data);
    }

    pub fn vertex_sub_data<T>(&mut self, vbo: &VertexBufferHandle, data: &[T], offset: uint) {
        let vbo = vbo.access();
        self.shared_state.borrow_mut().vbo_tracker.bind(vbo);
        vbo.sub_data(data, offset);
    }

    pub fn use_vertex_array(&mut self, vao: &VertexArrayHandle) {
        self.shared_state.borrow_mut().vao_tracker.bind_for_drawing(vao.access())
    }

    pub fn use_program(&mut self, program: &ProgramHandle) {
        program.access().use_program();
    }

    pub fn draw_arrays(&mut self, first: u32, count: u32) {
        self.prepare_for_drawing();
        draw::draw_arrays(first, count);
    }

    pub fn clear(&mut self) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        check_error!();
    }

    pub fn set_option(&mut self, option: RenderOption) {
        options::set_option(option);
    }

    fn prepare_for_drawing(&mut self) {
        self.shared_state.borrow_mut().vao_tracker.prepare_for_drawing();
    }

    fn bind_vbo_for_editing(&mut self, vbo: &VertexBufferHandle) {
        let vbo = vbo.access();
        self.shared_state.borrow_mut().vbo_tracker.bind(vbo);
    }

    fn bind_vao_for_editing(&mut self, vao: &VertexArray) {
        self.shared_state.borrow_mut().vao_tracker.bind_for_editing(vao);
    }
}

#[unsafe_destructor]
impl Drop for Context {
    fn drop(&mut self) {
        self.shared_state.borrow_mut().is_alive = false;
    }
}