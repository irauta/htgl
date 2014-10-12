
#![feature(unsafe_destructor,macro_rules,slicing_syntax,if_let)]

extern crate core;
extern crate gl;

pub use gl::load_with;
pub use renderer::Renderer;
pub use program::ProgramEditor;
pub use buffer::vertexbuffer::VertexBufferEditor;
pub use buffer::indexbuffer::IndexBufferEditor;
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

pub type VertexBufferHandle = Handle<buffer::VertexBuffer>;
pub type IndexBufferHandle = Handle<buffer::IndexBuffer>;
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
