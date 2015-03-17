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

#![feature(unsafe_destructor,alloc)]

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
    SimpleUniformTypeInt,
    SimpleUniformTypeMatrix,
    SimpleUniformTypeusize};
pub use shader::ShaderInfoAccessor;
pub use buffer::vertexbuffer::VertexBufferEditor;
pub use buffer::indexbuffer::IndexBufferEditor;
pub use buffer::uniformbuffer::UniformBufferEditor;
pub use context::Context;
pub use vertexarray::VertexAttributeType;
pub use options::RenderOption;
pub use renderer::PrimitiveMode;

use vertexarray::VertexArray;
use program::Program;
use buffer::vertexbuffer::VertexBuffer;
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

pub type VertexBufferHandle = Handle<buffer::VertexBuffer>;
pub type IndexBufferHandle = Handle<buffer::IndexBuffer>;
pub type UniformBufferHandle = Handle<buffer::UniformBuffer>;
pub type VertexArrayHandle = Handle<vertexarray::VertexArray>;
pub type ShaderHandle = Handle<shader::Shader>;
pub type ProgramHandle = Handle<program::Program>;
