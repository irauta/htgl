
#![feature(unsafe_destructor,macro_rules,slicing_syntax,if_let)]

extern crate core;
extern crate gl;

pub use gl::load_with;
pub use renderer::Renderer;
pub use program::ProgramEditor;
pub use buffer::vertexbuffer::VertexBufferEditor;
pub use buffer::indexbuffer::IndexBufferEditor;
pub use buffer::uniformbuffer::UniformBufferEditor;
pub use context::Context;

use vertexarray::VertexArray;
use program::Program;
use buffer::vertexbuffer::VertexBuffer;
use handle::Handle;

macro_rules! check_error(
    () => (::util::check_error(file!(), line!()));
)

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


pub enum PrimitiveMode {
    Triangles
}

pub enum RenderOption {
    ClearColor(f32, f32, f32, f32),
    DepthTest(bool),
    CullingEnabled(bool)
}

#[deriving(Clone,Show)]
pub enum AttributeType {
    AttributeByte,
    AttributeUnsignedByte,
    AttributeShort,
    AttributeUnsignedShort,
    AttributeInt,
    AttributeUnsignedInt,
    AttributeHalfFloat,
    AttributeFloat,
    AttributeDouble,
    AttributeInt2101010Rev,
    AttributeUnsignedInt2101010Rev
}

pub enum ShaderType {
    VertexShader,
    FragmentShader
}

#[deriving(Show)]
pub enum SimpleUniformTypeFloat {
    SimpleUniform1f,
    SimpleUniform2f,
    SimpleUniform3f,
    SimpleUniform4f
}

#[deriving(Show)]
pub enum SimpleUniformTypeMatrix {
    SimpleUniformMatrix2f,
    SimpleUniformMatrix3f,
    SimpleUniformMatrix4f,
    SimpleUniformMatrix2x3f,
    SimpleUniformMatrix3x2f,
    SimpleUniformMatrix2x4f,
    SimpleUniformMatrix4x2f,
    SimpleUniformMatrix3x4f,
    SimpleUniformMatrix4x3f
}

#[deriving(Show)]
pub enum SimpleUniformTypeInt {
    SimpleUniform1i,
    SimpleUniform2i,
    SimpleUniform3i,
    SimpleUniform4i
}

#[deriving(Show)]
pub enum SimpleUniformTypeUint {
    SimpleUniform1u,
    SimpleUniform2u,
    SimpleUniform3u,
    SimpleUniform4u
}
