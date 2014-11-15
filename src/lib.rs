
#![feature(unsafe_destructor,macro_rules,slicing_syntax,if_let)]

extern crate core;
extern crate gl;

pub use gl::load_with;
pub use renderer::Renderer;
pub use program::{ProgramEditor,ProgramInfoAccessor};
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

#[deriving(Show)]
pub enum UniformType {
    UniformFloat,
    UniformFloatVec2,
    UniformFloatVec3,
    UniformFloatVec4,
    UniformInt,
    UniformIntVec2,
    UniformIntVec3,
    UniformIntVec4,
    UniformUnsignedInt,
    UniformUnsignedIntVec2,
    UniformUnsignedIntVec3,
    UniformUnsignedIntVec4,
    UniformBool,
    UniformBoolVec2,
    UniformBoolVec3,
    UniformBoolVec4,
    UniformFloatMat2,
    UniformFloatMat3,
    UniformFloatMat4,
    UniformFloatMat2x3,
    UniformFloatMat2x4,
    UniformFloatMat3x2,
    UniformFloatMat3x4,
    UniformFloatMat4x2,
    UniformFloatMat4x3,
    UniformSampler1d,
    UniformSampler2d,
    UniformSampler3d,
    UniformSamplerCube,
    UniformSampler1dShadow,
    UniformSampler2dShadow,
    UniformSampler1dArray,
    UniformSampler2dArray,
    UniformSampler1dArrayShadow,
    UniformSampler2dArrayShadow,
    UniformSampler2dMultisample,
    UniformSampler2dMultisampleArray,
    UniformSamplerCubeShadow,
    UniformSamplerBuffer,
    UniformSampler2dRect,
    UniformSampler2dRectShadow,
    UniformIntSampler1d,
    UniformIntSampler2d,
    UniformIntSampler3d,
    UniformIntSamplerCube,
    UniformIntSampler1dArray,
    UniformIntSampler2dArray,
    UniformIntSampler2dMultisample,
    UniformIntSampler2dMultisampleArray,
    UniformIntSamplerBuffer,
    UniformIntSampler2dRect,
    UniformUnsignedIntSampler1d,
    UniformUnsignedIntSampler2d,
    UniformUnsignedIntSampler3d,
    UniformUnsignedIntSamplerCube,
    UniformUnsignedIntSampler1dArray,
    UniformUnsignedIntSampler2dArray,
    UniformUnsignedIntSampler2dMultisample,
    UniformUnsignedIntSampler2dMultisampleArray,
    UniformUnsignedIntSamplerBuffer,
    UniformUnsignedIntSampler2dRect
}