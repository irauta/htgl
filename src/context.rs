
use core::cell::RefCell;
use std::rc::Rc;

use super::{ShaderType,VertexAttributeType};
use super::{VertexBufferHandle,IndexBufferHandle,UniformBufferHandle,VertexArrayHandle,ProgramHandle,ShaderHandle};
use super::handle::{new_handle,HandleAccess};
use super::program::{mod,Program,ProgramEditor,ProgramInfoAccessor};
use super::shader::{mod,Shader,ShaderInfoAccessor};
use super::buffer;
use super::buffer::vertexbuffer::{VertexBuffer,VertexBufferEditor};
use super::buffer::uniformbuffer::{UniformBuffer,UniformBufferEditor};
use super::buffer::indexbuffer::IndexBufferEditor;
use super::vertexarray::{VertexArray,VertexAttribute};
use super::renderer::Renderer;
use super::tracker::{SimpleBindingTracker,RenderBindingTracker,TrackerIdGenerator};
use super::info::{ContextInfo,build_info};

pub struct Context {
    info: ContextInfo,
    id_generator: TrackerIdGenerator,
    program_tracker: RenderBindingTracker<Program>,
    vbo_tracker: SimpleBindingTracker<VertexBuffer>,
    ubo_tracker: SimpleBindingTracker<UniformBuffer>,
    vao_tracker: RenderBindingTracker<VertexArray>,
    shared_state: Rc<RefCell<SharedContextState>>
}

impl Context {
    pub fn new() -> Context {
        Context {
            info: build_info(),
            id_generator: TrackerIdGenerator::new(),
            program_tracker: RenderBindingTracker::new(),
            vbo_tracker: SimpleBindingTracker::new(),
            ubo_tracker: SimpleBindingTracker::new(),
            vao_tracker: RenderBindingTracker::new(),
            shared_state: Rc::new(RefCell::new(SharedContextState::new()))
        }
    }

    // Construct new objects

    pub fn new_vertex_buffer(&mut self) -> VertexBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(buffer::vertexbuffer::new_vertex_buffer(id, registration))
    }

    pub fn new_index_buffer(&mut self) -> IndexBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(buffer::indexbuffer::new_index_buffer(id, registration))
    }

    pub fn new_uniform_buffer(&mut self) -> UniformBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(buffer::uniformbuffer::new_uniform_buffer(id, registration))
    }

    pub fn new_vertex_array(&mut self,
                            attributes: &[VertexAttribute],
                            index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(VertexArray::new(self, id, attributes, index_buffer, registration))
    }

    pub fn new_vertex_array_simple(&mut self,
                                   attributes: &[(u8, VertexAttributeType, bool)],
                                   vertex_buffer: VertexBufferHandle,
                                   index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(VertexArray::new_single_vbo(self, id, attributes, vertex_buffer, index_buffer, registration))
    }

    pub fn new_shader(&mut self, shader_type: ShaderType, source: &str) -> ShaderHandle {
        let registration = self.registration_handle();
        new_handle(Shader::new(shader_type, source, registration))
    }

    pub fn new_program(&mut self, shaders: &[ShaderHandle]) -> ProgramHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(Program::new(id, shaders, registration))
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

    pub fn edit_uniform_buffer<'a>(&'a mut self, ubo: &'a UniformBufferHandle) -> UniformBufferEditor {
        buffer::uniformbuffer::new_uniform_buffer_editor(self, ubo.access())
    }

    pub fn edit_program<'a>(&'a mut self, program: &'a ProgramHandle) -> ProgramEditor {
        program::new_program_editor(self, program.access())
    }

    pub fn program_info<'a>(&'a self, program: &'a ProgramHandle) -> ProgramInfoAccessor {
        program::new_program_info_accessor(program.access())
    }

    pub fn shader_info<'a>(&'a self, shader: &'a ShaderHandle) -> ShaderInfoAccessor {
        shader::new_shader_info_accessor(shader.access())
    }

    // Commands that do not (directly) consume resources

    pub fn renderer<'a>(&'a mut self) -> Renderer {
        Renderer::new(self)
    }

    // Expose context info to user too!
    pub fn get_info(&self) -> &ContextInfo {
        &self.info
    }

    // Internal stuff

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

pub trait ContextEditingSupport {
    fn bind_vbo_for_editing(&mut self, vbo: &VertexBuffer);
    fn bind_ubo_for_editing(&mut self, vbo: &UniformBuffer);
    fn bind_vao_for_editing(&mut self, vao: &VertexArray);
    fn bind_program_for_editing(&mut self, program: &Program);
}

impl ContextEditingSupport for Context {
    fn bind_vbo_for_editing(&mut self, vbo: &VertexBuffer) {
        self.vbo_tracker.bind(vbo);
    }

    fn bind_ubo_for_editing(&mut self, ubo: &UniformBuffer) {
        self.ubo_tracker.bind(ubo);
    }

    fn bind_vao_for_editing(&mut self, vao: &VertexArray) {
        self.vao_tracker.bind_for_editing(vao);
    }

    fn bind_program_for_editing(&mut self, program: &Program) {
        self.program_tracker.bind_for_editing(program);
    }
}

pub trait ContextRenderingSupport {
    fn bind_vao_for_rendering(&mut self, vao: &VertexArrayHandle);
    fn bind_program_for_rendering(&mut self, program: &ProgramHandle);
    fn prepare_for_rendering(&mut self);
}

impl ContextRenderingSupport for Context {
    fn bind_vao_for_rendering(&mut self, vao: &VertexArrayHandle) {
        self.vao_tracker.bind_for_rendering(vao.rc());
    }

    fn bind_program_for_rendering(&mut self, program: &ProgramHandle) {
        self.program_tracker.bind_for_rendering(program.rc());
    }

    fn prepare_for_rendering(&mut self) {
        self.vao_tracker.restore_rendering_state();
        self.program_tracker.restore_rendering_state();
    }
}


pub struct SharedContextState {
    pub context_alive: bool
}

impl SharedContextState {
    pub fn new() -> SharedContextState {
        SharedContextState {
            context_alive: true
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
        self.context_shared.borrow().context_alive
    }
}