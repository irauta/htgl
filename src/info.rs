
use gl;
use gl::types::{GLint,GLenum};

#[deriving(Show)]
pub struct ContextInfo {
    pub uniform_buffer: UniformBufferInfo
}

#[deriving(Show)]
pub struct UniformBufferInfo {
    pub max_bindings: GLint,
    pub max_geometry_blocks: GLint,
    pub max_vertex_blocks: GLint,
    pub max_fragment_blocks: GLint,
    pub max_block_size: GLint,
    pub offset_alignment: GLint
}

pub fn build_info() -> ContextInfo {
    ContextInfo {
        uniform_buffer: UniformBufferInfo {
            max_bindings: get_integer(gl::MAX_UNIFORM_BUFFER_BINDINGS),
            max_vertex_blocks: get_integer(gl::MAX_VERTEX_UNIFORM_BLOCKS),
            max_geometry_blocks: get_integer(gl::MAX_GEOMETRY_UNIFORM_BLOCKS),
            max_fragment_blocks: get_integer(gl::MAX_FRAGMENT_UNIFORM_BLOCKS),
            max_block_size: get_integer(gl::MAX_UNIFORM_BLOCK_SIZE),
            offset_alignment: get_integer(gl::UNIFORM_BUFFER_OFFSET_ALIGNMENT)
        }
    }
}

fn get_integer(property: GLenum) -> GLint {
    unsafe {
        let mut value = 0;
        gl::GetIntegerv(property, &mut value);
        check_error!();
        value
    }
}