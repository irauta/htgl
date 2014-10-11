
#![feature(unsafe_destructor,macro_rules,slicing_syntax,if_let)]

extern crate core;
extern crate gl;

pub use gl::load_with;
pub use vertexarray::{VertexAttribute,AttributeType,AttributeByte,AttributeUnsignedByte,AttributeShort,AttributeUnsignedShort,AttributeInt,AttributeUnsignedInt,AttributeHalfFloat,AttributeFloat,AttributeDouble,AttributeInt2101010Rev,AttributeUnsignedInt2101010Rev};
pub use shader::{ShaderType,VertexShader,FragmentShader};
pub use options::{RenderOption,ClearColor,DepthTest,CullingEnabled};
pub use renderer::{Renderer,PrimitiveMode,Triangles};
pub use program::ProgramEditor;
pub use buffer::vertexbuffer::VertexBufferEditor;
pub use buffer::indexbuffer::IndexBufferEditor;

use core::cell::RefCell;
use std::rc::Rc;
use vertexarray::VertexArray;
use context::{SharedContextState,RegistrationHandle};
use tracker::{SimpleBindingTracker,RenderBindingTracker,TrackerIdGenerator,TrackerId};
use program::Program;
use buffer::vertexbuffer::VertexBuffer;

macro_rules! check_error(
    () => (::util::check_error(file!(), line!()));
)

mod buffer;
mod util;
mod tracker;
mod vertexarray;
mod shader;
mod program;
mod options;
mod renderer;
mod context;

pub type VertexBufferHandle = Handle<buffer::VertexBuffer>;
pub type IndexBufferHandle = Handle<buffer::IndexBuffer>;
pub type VertexArrayHandle = Handle<vertexarray::VertexArray>;
pub type ShaderHandle = Handle<shader::Shader>;
pub type ProgramHandle = Handle<program::Program>;


trait Bind {
    fn bind(&self);
    fn get_id(&self) -> TrackerId;
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

    fn rc(&self) -> &Rc<T> {
        &self.resource
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Handle<T> {
        Handle { resource: self.resource.clone() }
    }
}


pub struct Context {
    id_generator: TrackerIdGenerator,
    program_tracker: RenderBindingTracker<Program>,
    vbo_tracker: SimpleBindingTracker<VertexBuffer>,
    vao_tracker: RenderBindingTracker<VertexArray>,
    shared_state: Rc<RefCell<SharedContextState>>
}

impl Context {
    pub fn new() -> Context {
        Context {
            id_generator: TrackerIdGenerator::new(),
            program_tracker: RenderBindingTracker::new(),
            vbo_tracker: SimpleBindingTracker::new(),
            vao_tracker: RenderBindingTracker::new(),
            shared_state: Rc::new(RefCell::new(SharedContextState::new()))
        }
    }

    // Construct new objects

    pub fn new_vertex_buffer(&mut self) -> VertexBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        Handle::new(buffer::vertexbuffer::new_vertex_buffer(id, registration))
    }

    pub fn new_index_buffer(&mut self) -> IndexBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        Handle::new(buffer::indexbuffer::new_index_buffer(id, registration))
    }

    pub fn new_vertex_array(&mut self,
                            attributes: &[VertexAttribute],
                            index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        Handle::new(vertexarray::VertexArray::new(self, id, attributes, index_buffer, registration))
    }

    pub fn new_vertex_array_simple(&mut self,
                                   attributes: &[(u8, AttributeType, bool)],
                                   vertex_buffer: VertexBufferHandle,
                                   index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        Handle::new(vertexarray::VertexArray::new_single_vbo(self, id, attributes, vertex_buffer, index_buffer, registration))
    }

    pub fn new_shader(&mut self, shader_type: ShaderType, source: &str) -> ShaderHandle {
        let registration = self.registration_handle();
        Handle::new(shader::Shader::new(shader_type, source, registration))
    }

    pub fn new_program(&mut self, shaders: &[ShaderHandle]) -> ProgramHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        Handle::new(program::Program::new(id, shaders, registration))
    }

    // Modify object contents with the help of editor objects

    pub fn edit_vertex_buffer<'a>(&'a mut self, vbo: &'a VertexBufferHandle) -> VertexBufferEditor {
        buffer::vertexbuffer::new_vertex_buffer_editor(self, vbo.access())
    }

    pub fn edit_index_buffer<'a>(&'a mut self, vao: &'a VertexArrayHandle) -> Option<IndexBufferEditor> {
        let vao = vao.access();
        match vao.index_buffer() {
            Some(_) => Some(buffer::indexbuffer::new_index_buffer_editor(self, vao)),
            None => None
        }
    }

    pub fn edit_program<'a>(&'a mut self, program: &'a ProgramHandle) -> ProgramEditor {
        program::new_program_editor(self, program.access())
    }

    // Commands that do not (directly) consume resources

    pub fn renderer<'a>(&'a mut self) -> Renderer {
        Renderer::new(self)
    }

    // Internal stuff

    fn bind_vbo_for_editing(&mut self, vbo: &VertexBuffer) {
        self.vbo_tracker.bind(vbo);
    }

    fn bind_vao_for_editing(&mut self, vao: &VertexArray) {
        self.vao_tracker.bind_for_editing(vao);
    }

    fn bind_vao_for_rendering(&mut self, vao: &VertexArrayHandle) {
        self.vao_tracker.bind_for_rendering(vao.rc());
    }

    fn bind_program_for_editing(&mut self, program: &Program) {
        self.program_tracker.bind_for_editing(program);
    }

    fn bind_program_for_rendering(&mut self, program: &ProgramHandle) {
        self.program_tracker.bind_for_rendering(program.rc());
    }

    fn prepare_for_rendering(&mut self) {
        self.vao_tracker.restore_rendering_state();
        self.program_tracker.restore_rendering_state();
    }

    fn registration_handle(&self) -> RegistrationHandle {
        RegistrationHandle::new(self.shared_state.clone())
    }
}

#[unsafe_destructor]
impl Drop for Context {
    fn drop(&mut self) {
        self.shared_state.borrow_mut().context_alive = false;
    }
}


#[deriving(Show)]
pub enum UniformTypeFloat {
    Uniform1f,
    Uniform2f,
    Uniform3f,
    Uniform4f
}

#[deriving(Show)]
pub enum UniformTypeMatrix {
    UniformMatrix2f,
    UniformMatrix3f,
    UniformMatrix4f,
    UniformMatrix2x3f,
    UniformMatrix3x2f,
    UniformMatrix2x4f,
    UniformMatrix4x2f,
    UniformMatrix3x4f,
    UniformMatrix4x3f
}

#[deriving(Show)]
pub enum UniformTypeInt {
    Uniform1i,
    Uniform2i,
    Uniform3i,
    Uniform4i
}

#[deriving(Show)]
pub enum UniformTypeUint {
    Uniform1u,
    Uniform2u,
    Uniform3u,
    Uniform4u
}
