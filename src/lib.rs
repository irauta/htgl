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

//! # Intro
//!
//! The purpose of this crate is to wrap the low-level OpenGL functions with safer abstraction
//! that uses Rust's capabilities to make it easier to build correctly working programs. The
//! preferred way is to ensure validity statically, but runtime checks are used where necessary.
//! That said, OpenGL error checking is meant to be enabled only in debug builds, as the basic
//! error checking with glGetError may cause slowdowns.
//!
//! The second purpose is to make OpenGL more convenient to use. Non-purposes or non-goals include
//! hiding OpenGL - while there are some new abstractions, the underlying API is not meant to be
//! abstracted away. Another non-goal is to make this library a complete wrapper for OpenGL.
//!
//! # The main principles
//!
//! The central concept is the context, represented by the struct `Context`. Everything else is
//! ultimately done through it. Even though it doesn't have protections against multiple
//! instantiation, you should create just one (per actual OpenGL context). It's not `Copy` or
//! even `Clone`, so accidental copies shouldn't happen.
//!
//! The resources (textures, buffers, vertex arrays and so on) are handled indirectly through
//! handle objects. Just pass them to the methods of `Context` (or the related structs) when
//! you want to do something with them.
//!
//! ## Editing and rendering resources
//!
//! The context is used in a "modal" way. This is the most obvious difference this library has with
//! vanilla OpenGL.
//!
//! If you want to manipulate vertex buffer contents, you get youself a buffer editor via the
//! `edit_vertex_buffer` method of Context that in turn allows you to call `data` or `sub_data` to
//! set the contents. The `edit_vertex_buffer` takes `&mut self` as its first parameter, so there
//! can exist only a single editor object at a time.
//!
//! The idea behind this is to make sure that the OpenGL context has only one resource bound to
//! a single binding point at a time, the borrow checker can easily verify at compile time that
//! there are no cases where there is only a single glBind* call but attempts to edit two different
//! resources of same type.
//!
//! The code can get more verbose, because the borrow checker makes sure multiple resources of any
//! type won't be edited at once, but the rigidity is hopefully worth it.
//!
//! There is also a "rendering mode", accessed through the contexts's `renderer` method. You can
//! bind resources to the context more freely in the rendering mode, and the resources bound last
//! before the actual drawing command are the ones that are going to be used. The term is "use"
//! instead of "bind" in the method names, for example use_vertex_array, to keep some distance to
//! the low level bind commands. The use methods try to make sure resources already bound wouldn't
//! be bound again - the supposition is that it's better to do a very simple comparison every time
//! instead of a more expensive call to the driver every time. Time will tell if this is a good
//! idea.

extern crate gl;

pub use gl::load_with;
pub use renderer::Renderer;
pub use shader::ShaderType;
pub use program::{ProgramEditor,
    ProgramInfoAccessor,
    ShaderAttributeInfo,
    ShaderAttribute,
    UniformInfo,
    Uniform,
    InterfaceBlock,
    BlockUniform,
    SimpleUniformTypeFloat,
    SimpleUniformTypeI32,
    SimpleUniformTypeMatrix,
    SimpleUniformTypeU32};
pub use shader::ShaderInfoAccessor;
pub use buffer::BufferEditor;
pub use context::Context;
pub use vertexarray::VertexAttributeType;
pub use options::RenderOption;
pub use renderer::PrimitiveMode;

use vertexarray::VertexArray;
use program::Program;
use handle::Handle;

macro_rules! check_error(
    () => (::util::check_error(file!(), line!()));
);

mod handle;
mod buffer;
mod util;
mod tracker;
mod vertexarray;
mod shader;
mod program;
mod options;
mod renderer;
mod context;
mod info;

/// Handle to a buffer object (vertex, index, uniform and so on).
pub type BufferHandle = Handle<buffer::BufferObject>;
/// Handle to a vertex array object.
pub type VertexArrayHandle = Handle<vertexarray::VertexArray>;
/// Handle to a shader object. Shaders are not used for rendering themselves, they are linked into
/// program objects.
pub type ShaderHandle = Handle<shader::Shader>;
/// Handle to a shader program.
pub type ProgramHandle = Handle<program::Program>;
