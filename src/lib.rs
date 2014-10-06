
#![feature(unsafe_destructor,macro_rules,slicing_syntax,if_let)]

extern crate core;
extern crate gl;

pub use gl::load_with;
pub use vertexarray::{VertexAttribute,AttributeType,AttributeByte,AttributeUnsignedByte,AttributeShort,AttributeUnsignedShort,AttributeInt,AttributeUnsignedInt,AttributeHalfFloat,AttributeFloat,AttributeDouble,AttributeInt2101010Rev,AttributeUnsignedInt2101010Rev};
pub use shader::{ShaderType,VertexShader,FragmentShader};
pub use options::{RenderOption,ClearColor,DepthTest,CullingEnabled};
pub use renderer::{Renderer,PrimitiveMode,Triangles};
pub use editor::{VertexBufferEditor,IndexBufferEditor,ProgramEditor,UniformTypeFloat,Uniform1f,Uniform2f,Uniform3f,Uniform4f,UniformTypeMatrix,UniformMatrix2f,UniformMatrix3f,UniformMatrix4f,UniformMatrix2x3f,UniformMatrix3x2f,UniformMatrix2x4f,UniformMatrix4x2f,UniformMatrix3x4f,UniformMatrix4x3f,UniformTypeInt,Uniform1i,Uniform2i,Uniform3i,Uniform4i,UniformTypeUint,Uniform1u,Uniform2u,Uniform3u,Uniform4u};

use core::cell::RefCell;
use std::rc::Rc;
use buffer::VertexBuffer;
use vertexarray::VertexArray;
use context::{SharedContextState,RegistrationHandle};
use tracker::{SimpleBindingTracker,TrackerIdGenerator,TrackerId};
use shader::Program;

macro_rules! check_error(
    () => (::util::check_error(file!(), line!()));
)

mod buffer;
mod util;
mod tracker;
mod vertexarray;
mod shader;
mod options;
mod renderer;
mod context;
mod editor;

pub type VertexBufferHandle = Handle<buffer::VertexBuffer>;
pub type IndexBufferHandle = Handle<buffer::IndexBuffer>;
pub type VertexArrayHandle = Handle<vertexarray::VertexArray>;
pub type ShaderHandle = Handle<shader::Shader>;
pub type ProgramHandle = Handle<shader::Program>;


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
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Handle<T> {
        Handle { resource: self.resource.clone() }
    }
}


pub struct Context {
    id_generator: TrackerIdGenerator,
    program_tracker: SimpleBindingTracker<Program>,
    vbo_tracker: SimpleBindingTracker<VertexBuffer>,
    vao_tracker: SimpleBindingTracker<VertexArray>,
    shared_state: Rc<RefCell<SharedContextState>>
}

impl Context {
    pub fn new() -> Context {
        Context {
            id_generator: TrackerIdGenerator::new(),
            program_tracker: SimpleBindingTracker::new(),
            vbo_tracker: SimpleBindingTracker::new(),
            vao_tracker: SimpleBindingTracker::new(),
            shared_state: Rc::new(RefCell::new(SharedContextState::new()))
        }
    }

    // Construct new objects

    pub fn new_vertex_buffer(&mut self) -> VertexBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        Handle::new(buffer::new_vertex_buffer(id, registration))
    }

    pub fn new_index_buffer(&mut self) -> IndexBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        Handle::new(buffer::new_index_buffer(id, registration))
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
        Handle::new(shader::Program::new(id, shaders, registration))
    }

    // Modify object contents with the help of editor objects

    pub fn edit_vertex_buffer<'a>(&'a mut self, vbo: &'a VertexBufferHandle) -> VertexBufferEditor {
        editor::new_vertex_buffer_editor(self, vbo.access())
    }

    pub fn edit_index_buffer<'a>(&'a mut self, vao: &'a VertexArrayHandle) -> Option<IndexBufferEditor> {
        let vao = vao.access();
        match vao.index_buffer() {
            Some(_) => Some(editor::new_index_buffer_editor(self, vao)),
            None => None
        }
    }

    pub fn edit_program<'a>(&'a mut self, program: &'a ProgramHandle) -> ProgramEditor {
        editor::new_program_editor(self, program.access())
    }

    // Commands that do not (directly) consume resources

    pub fn renderer<'a>(&'a mut self) -> Renderer {
        Renderer::new(self)
    }

    // Internal stuff

    fn bind_vbo(&mut self, vbo: &VertexBuffer) {
        self.vbo_tracker.bind(vbo);
    }

    fn bind_vao(&mut self, vao: &VertexArray) {
        self.vao_tracker.bind(vao);
    }

    fn bind_program(&mut self, program: &Program) {
        self.program_tracker.bind(program);
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