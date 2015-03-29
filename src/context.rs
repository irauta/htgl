// Copyright 2015 Ilkka Rauta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! See the struct `Context` for documentation on how the context is meant to be used.

use std::cell::RefCell;
use std::rc::Rc;

use super::{VertexBufferHandle,IndexBufferHandle,UniformBufferHandle,VertexArrayHandle,ProgramHandle,ShaderHandle};
use super::handle::{new_handle,HandleAccess};
use super::program::{self,Program,ProgramEditor,ProgramInfoAccessor};
use super::shader::{self,Shader,ShaderInfoAccessor,ShaderType};
use super::buffer;
use super::buffer::vertexbuffer::{VertexBuffer,VertexBufferEditor};
use super::buffer::uniformbuffer::{UniformBuffer,UniformBufferEditor};
use super::buffer::indexbuffer::IndexBufferEditor;
use super::vertexarray::{VertexArray,VertexAttribute,VertexAttributeType};
use super::renderer::Renderer;
use super::tracker::{SimpleBindingTracker,RenderBindingTracker,TrackerIdGenerator};
use super::info::{ContextInfo,build_info};

/// Context is a central concept in OpenGL, even though it's not a concrete item in the GL API.
/// This struct is meant to be a stand-in for the GL context, but also the starting point for all
/// the functionality this library offers.
///
/// # Contexts
///
/// Even though you're not prevented from instantiating more than one Contexts, you most likely
/// shouldn't. If you do, and use both contexts when only one actual OpenGL context is active,
/// things will not work as intended. This is an intentional trade-off. As mentioned, OpenGL API
/// does not expose the context in any explicit way, but this is in practice a property of the
/// windowing system. (For example, see wglMakeCurrent and glXMakeCurrent.) In addition, the
/// contexts may very well be managed by a wrapper library, abstracting away all the low-level and
/// platform specific peculiarities. This library would have to either work with only one of them,
/// many of them, or force you to make some kind of integration between this library and the other
/// library, giving the burden to you anyway. So, the main point still is: just don't create more
/// than one Context and you should be fine.
///
/// To create a Context object, call `Context::new()`.
///
/// # Resources
///
/// To create other objects, like vertex buffers or shaders, use the other new_-prefixed functions.
/// For example, to create a vertex buffer, call `ctx.new_vertex_buffer()`.
///
/// The resources are not returned themselves, but are always accessed through handles. This is
/// somewhat analogous with the way OpenGL API itself works. The main difference is that there is
/// no explicit bind method in this library, instead to edit an vertex buffer you call the
/// `edit_vertex_buffer` method of Context:
///
///    let vbo = ctx.new_vertex_buffer();
///    {
///        let editor = ctx.edit_vertex_buffer(&vbo);
///        editor.data(&vertex_data)
///    }
///
/// A shorter version with chaining:
///
///    let vbo = ctx.new_vertex_buffer();
///    ctx.edit_vertex_buffer(&vbo).data(&vertices);
///
/// ## Resource handles
///
/// The handles can be cloned and extend the lifetime of the actual resource in the same way
/// `std::rc::Rc` does. This is because some OpenGL resources may "embed" a resource in themselves.
/// And example of this is vertex arrays and index buffers. An index buffer may be attached to many
/// vertex arrays, and it needs to live as long as any of the vertex arrays live.
///
/// ## Editing is exclusive
///
/// You should notice that the edit_* functions borrow the context mutably, that is, there can
/// exist only one editor object at once. This is to ensure at compile time that the functions that
/// actually edit the one resource (for example `data` in the example above) specified. After all,
/// the underlying API doesn't allow having multiple vertex buffers bound at once, for example.
/// (The rules are more complicated for textures as there are several slots for textures.) The
/// limitation also works as a way to minimize the actual glBind* calls within the library.
///
/// ## Rendering mode
///
/// While binding resources is otherwise not explicitly exposed through the API, there is a
/// "rendering mode" that also borrows the context mutably, thus you can't issue rendering commands
/// and editing commands intertwined, at least not without creating (and destroying) editor and
/// renderer objects in large amounts. You can access the rendering mode by calling the
/// `renderer()` method. The `Renderer` struct has use_* methods that are analogous to the glBind*
/// functions, and also naturally the draw methods et cetera. See the Renderer documentation for
/// more about rendering.
pub struct Context {
    info: ContextInfo,
    id_generator: TrackerIdGenerator,
    /// The more costly and complex tracker is used, because programs might be edited while
    /// rendering - namely the uniforms and attributes.
    program_tracker: RenderBindingTracker<Program>,
    vbo_tracker: SimpleBindingTracker<VertexBuffer>,
    ubo_tracker: SimpleBindingTracker<UniformBuffer>,
    vao_tracker: RenderBindingTracker<VertexArray>,
    /// Shared state is a way for context to communicate things to resources - mainly that the
    /// context is alive (or is not)
    shared_state: Rc<RefCell<SharedContextState>>
}

impl Context {
    /// Creates a new Context. Do not create more than one (per actual OpenGL context, anyway).
    /// See the documentation for the struct for more details on what creating a `Context` means.
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

    /// Create a new vertex buffer object.
    ///
    /// Returns a handle to the created vertex buffer.
    pub fn new_vertex_buffer(&mut self) -> VertexBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(buffer::vertexbuffer::new_vertex_buffer(id, registration))
    }

    /// Create a new index buffer object.
    ///
    /// Returns a handle to the created index buffer.
    pub fn new_index_buffer(&mut self) -> IndexBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(buffer::indexbuffer::new_index_buffer(id, registration))
    }

    /// Create a new uniform buffer object.
    ///
    /// Returns a handle to the created uniform buffer.
    pub fn new_uniform_buffer(&mut self) -> UniformBufferHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(buffer::uniformbuffer::new_uniform_buffer(id, registration))
    }

    /// Create a new vertex array object.
    ///
    /// See the `glVertexAttribPointer` documentation for how the attributes are specified.
    /// This function takes a slice of vertex attributes at once - the created vertex array
    /// is immutable, you can't change the attributes afterwards!
    ///
    /// If an index buffer should be associated with the vertex array, give a handle to it as the
    /// third argument.
    pub fn new_vertex_array(&mut self,
                            attributes: &[VertexAttribute],
                            index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(VertexArray::new(self, id, attributes, index_buffer, registration))
    }

    /// Create a new vertex array object that only uses contents of one vertex buffer.
    ///
    /// See the `glVertexAttribPointer` documentation for how the attributes slice works.
    /// Otherwise, see the `new_vertex_array` documentation. The only vertex array the vertex
    /// attributes refer to, are specified with the vertex_buffer argument.
    pub fn new_vertex_array_simple(&mut self,
                                   attributes: &[(u8, VertexAttributeType, bool)],
                                   vertex_buffer: VertexBufferHandle,
                                   index_buffer: Option<IndexBufferHandle>) -> VertexArrayHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(VertexArray::new_single_vbo(self, id, attributes, vertex_buffer, index_buffer, registration))
    }

    /// Create and compile a new shader object.
    pub fn new_shader(&mut self, shader_type: ShaderType, source: &str) -> ShaderHandle {
        let registration = self.registration_handle();
        new_handle(Shader::new(shader_type, source, registration))
    }

    /// Create and link a shader program from the specified shaders.
    pub fn new_program(&mut self, shaders: &[ShaderHandle]) -> ProgramHandle {
        let registration = self.registration_handle();
        let id = self.id_generator.new_id();
        new_handle(Program::new(id, shaders, registration))
    }

    // Modify object contents with the help of editor objects

    /// Edit a vertex buffer. Returns an editor object that can be used to modify the buffer
    /// contents.
    pub fn edit_vertex_buffer<'a>(&'a mut self, vbo: &'a VertexBufferHandle) -> VertexBufferEditor {
        buffer::vertexbuffer::new_vertex_buffer_editor(self, vbo.access())
    }

    /// Edit an index buffer. Returns an editor object that can be used to modify the buffer
    /// contents.
    ///
    /// Note that this function is given a *vertex array* instead of the index buffer directly.
    /// This is because the core OpenGL specification requires index buffers to be associated with
    /// an vertex array. The returned value is wrapped in an Option, because vertex arrays do not
    /// necessarily contain an index buffer. Still, it would be silly to call this function with a
    /// VAO that does not have an index buffer attached to it.
    pub fn edit_index_buffer<'a>(&'a mut self, vao: &'a VertexArrayHandle) -> Option<IndexBufferEditor> {
        let vao = vao.access();
        match vao.index_buffer() {
            Some(_) => Some(buffer::indexbuffer::new_index_buffer_editor(self, vao)),
            None => None
        }
    }

    /// Edit an uniform buffer. Returns an editor object that can be used to modify the buffer
    /// contents.
    pub fn edit_uniform_buffer<'a>(&'a mut self, ubo: &'a UniformBufferHandle) -> UniformBufferEditor {
        buffer::uniformbuffer::new_uniform_buffer_editor(self, ubo.access())
    }

    /// Lets you edit uniform bindings of a program with the returned editor.
    pub fn edit_program<'a>(&'a mut self, program: &'a ProgramHandle) -> ProgramEditor {
        program::new_program_editor(self, program.access())
    }

    /// Returns and "info accessor" that can figure out the attribute, uniform and fragment data
    /// locations and other related information.
    pub fn program_info<'a>(&'a self, program: &'a ProgramHandle) -> ProgramInfoAccessor {
        program::new_program_info_accessor(program.access())
    }

    /// Returns an "info accessor" that can tell if shader compilation succeeded and return the
    /// compilation info log.
    pub fn shader_info<'a>(&'a self, shader: &'a ShaderHandle) -> ShaderInfoAccessor {
        shader::new_shader_info_accessor(shader.access())
    }

    // Commands that do not (directly) consume resources

    /// Return a renderer object. See `Renderer` documentation for info on usage.
    pub fn renderer<'a>(&'a mut self) -> Renderer {
        Renderer::new(self)
    }

    // Expose context info to user too!

    /// `ContextInfo` contains unchanging values related to the context, like
    /// GL_MAX_UNIFORM_BUFFER_BINDINGS (the path is ContextInfo.uniform_buffer.max_bindings).
    /// Currently definitely limited.
    pub fn get_info(&self) -> &ContextInfo {
        &self.info
    }

    // Internal stuff

    /// Resources get a handle to the shared state
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

/// A trait with the purpose to expose only the editing functionality to other types in the
/// library, without exposing all the internals of `Context`. Specifically it facilitates
/// binding of resources *for editing*, something not exposed to outside users.
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

/// See `ContextEditingSupport`. This trait is to expose binding functions used when
/// *rendering* things.
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

/// Things that need to be shared between `Context` and the resources it spawns.
/// This might be a bad idea, but allows the resource handles to live longer than the context,
/// without causing freeing of GL resources after GL context has died. Alternative would have been
/// to limit lifetimes of resource handles to strictly live within the lifetime of the context, but
/// that would "infect" everything with a lifetime annotation...
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

/// Handle to the shared state, as used by the resources (and `Context`).
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