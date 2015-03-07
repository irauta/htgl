
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
