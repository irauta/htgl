
use std::ptr::null_mut;
use std::fmt::Show;

use gl;
use gl::types::GLenum;

use super::Program;
use super::super::{SimpleUniformTypeFloat,
    SimpleUniform1f,
    SimpleUniform2f,
    SimpleUniform3f,
    SimpleUniform4f,
    SimpleUniformTypeMatrix,
    SimpleUniformMatrix2f,
    SimpleUniformMatrix3f,
    SimpleUniformMatrix4f,
    SimpleUniformMatrix2x3f,
    SimpleUniformMatrix3x2f,
    SimpleUniformMatrix2x4f,
    SimpleUniformMatrix4x2f,
    SimpleUniformMatrix3x4f,
    SimpleUniformMatrix4x3f,
    SimpleUniformTypeInt,
    SimpleUniform1i,
    SimpleUniform2i,
    SimpleUniform3i,
    SimpleUniform4i,
    SimpleUniformTypeUint,
    SimpleUniform1u,
    SimpleUniform2u,
    SimpleUniform3u,
    SimpleUniform4u};

/// Helper struct containing all
struct GlUniform {
    name: String,
    uniform_type: i32,
    size: i32,
    block_index: i32,
    offset: i32,
    array_stride: i32,
    matrix_stride: i32,
}

impl GlUniform {
    fn new(name: String) -> GlUniform {
        GlUniform {
            name: name,
            uniform_type: 0,
            size: 0,
            block_index: 0,
            offset: 0,
            array_stride: 0,
            matrix_stride: 0
        }
    }
}

/// Top-level result structure for program's uniform introspection
#[deriving(Show)]
pub struct UniformInfo {
    pub globals: Vec<Uniform>,
    pub blocks: Vec<InterfaceBlock>
}

/// An uniform not in a block
#[deriving(Show)]
pub struct Uniform {
    pub name: String,
    pub location: i32,
    pub uniform_type: i32,
    pub size: i32
}

impl Uniform {
    fn new(gl_uniform: GlUniform, location: i32) -> Uniform {
        Uniform {
            name: gl_uniform.name,
            location: location,
            uniform_type: gl_uniform.uniform_type,
            size: gl_uniform.size
        }
    }
}

/// Description of an uniform block
#[deriving(Show)]
pub struct InterfaceBlock {
    pub name: String,
    pub index: u32,
    pub data_size: i32,
    pub uniforms: Vec<BlockUniform>
}

#[deriving(Show)]
pub struct BlockUniform {
    pub name: String,
    pub uniform_type: i32,
    pub size: i32,
    pub offset: i32,
    pub array_stride: i32,
    pub matrix_stride: i32,
}

impl BlockUniform {
    fn new(gl_uniform: GlUniform) -> BlockUniform {
        BlockUniform {
            name: gl_uniform.name,
            uniform_type: gl_uniform.uniform_type,
            size: gl_uniform.size,
            offset: gl_uniform.offset,
            array_stride: gl_uniform.array_stride,
            matrix_stride: gl_uniform.matrix_stride
        }
    }
}

pub fn make_uniform_info(program: &Program) -> UniformInfo {
    let gl_uniforms = make_gl_uniform_info_vec(program);
    let mut globals = Vec::new();
    let mut blocks = make_uniform_block_info_vec(program);
    for gl_uniform in gl_uniforms.into_iter() {
        if gl_uniform.block_index < 0 {
            let location = program.get_uniform_location(gl_uniform.name[]);
            globals.push(Uniform::new(gl_uniform, location));
        }
        else {
            let index = gl_uniform.block_index as uint;
            blocks[index].uniforms.push(BlockUniform::new(gl_uniform));
        }
    }
    UniformInfo {
        globals: globals,
        blocks: blocks
    }
}

fn make_gl_uniform_info_vec(program: &Program) -> Vec<GlUniform> {
    let count = program.get_value(gl::ACTIVE_UNIFORMS) as uint;
    if count == 0 {
        return Vec::new();
    }
    let mut info_vec = Vec::with_capacity(count);
    let mut intvalues = Vec::from_elem(count, 0);
    let indices = Vec::from_fn(count, |i| i as u32);
    fill_uniform_info_vec(program.id, &indices, gl::UNIFORM_NAME_LENGTH, &mut intvalues);
    for (index, expected_len) in intvalues.iter().enumerate() {
        info_vec.push(GlUniform::new(uniform_name(program.id, index as u32, *expected_len as u32)));
    }
    {
        let fill_info = |property, info_fn: |&mut GlUniform, i32|| {
            fill_uniform_info_vec(program.id, &indices, property, &mut intvalues);
            for (info, value) in info_vec.iter_mut().zip(intvalues.iter()) {
                info_fn(info, *value);
            }
        };
        fill_info(gl::UNIFORM_SIZE, |info, value| info.size = value);
        fill_info(gl::UNIFORM_TYPE, |info, value| info.uniform_type = value);
        fill_info(gl::UNIFORM_OFFSET, |info, value| info.offset = value);
        fill_info(gl::UNIFORM_BLOCK_INDEX, |info, value| info.block_index = value);
        fill_info(gl::UNIFORM_ARRAY_STRIDE, |info, value| info.array_stride = value);
        fill_info(gl::UNIFORM_MATRIX_STRIDE, |info, value| info.matrix_stride = value);
    }
    info_vec
}

fn make_uniform_block_info_vec(program: &Program) -> Vec<InterfaceBlock> {
    let count = program.get_value(gl::ACTIVE_UNIFORM_BLOCKS);
    if count == 0 {
        return Vec::new();
    }
    let mut info_vec = Vec::with_capacity(count as uint);
    for index in range(0, count as u32) {
        let expected_len = get_block_info(program.id, index, gl::UNIFORM_BLOCK_NAME_LENGTH) as u32;
        let data_size = get_block_info(program.id, index, gl::UNIFORM_BLOCK_DATA_SIZE);
        let name = block_name(program.id, index, expected_len);
        let index = get_uniform_block_index(program.id, name[]);
        info_vec.push(InterfaceBlock {
            index: index,
            name: name,
            data_size: data_size,
            uniforms: Vec::new()
        });
    }
    info_vec
}

fn fill_uniform_info_vec(program_id: u32, indices: &Vec<u32>, property: GLenum, intvalues: &mut Vec<i32>) {
    unsafe {
        gl::GetActiveUniformsiv(program_id, indices.len() as i32, indices.as_ptr(), property, intvalues.as_mut_ptr());
        check_error!();
    }
}

fn get_block_info(program_id: u32, block_index: u32, property: GLenum) -> i32 {
    unsafe {
        let mut value = 0;
        gl::GetActiveUniformBlockiv(program_id, block_index, property, &mut value);
        check_error!();
        value
    }
}

fn uniform_name(program_id: u32, index: u32, expected_len: u32) -> String {
    let mut name_vec = Vec::from_elem(expected_len as uint, 0u8);
    unsafe {
        let name_ptr = name_vec.as_mut_slice().as_mut_ptr() as *mut i8;
        gl::GetActiveUniformName(program_id, index, name_vec.len() as i32, null_mut(), name_ptr);
        check_error!();
    }
    name_vec.pop(); // Remove the null byte from end
    String::from_utf8(name_vec).unwrap()
}

fn block_name(program_id: u32, index: u32, expected_len: u32) -> String {
    let mut name_vec = Vec::from_elem(expected_len as uint, 0u8);
    unsafe {
        let name_ptr = name_vec.as_mut_slice().as_mut_ptr() as *mut i8;
        gl::GetActiveUniformBlockName(program_id, index, name_vec.len() as i32, null_mut(), name_ptr);
        check_error!();
    }
    name_vec.pop(); // Remove the null byte from end
    String::from_utf8(name_vec).unwrap()
}

fn get_uniform_block_index(program_id: u32, name: &str) -> u32 {
    let c_name = name.to_c_str();
    unsafe {
        let index = gl::GetUniformBlockIndex(program_id, c_name.as_ptr());
        check_error!();
        index
    }
}

pub fn uniform_f32(location: i32, count: uint, uniform_type: SimpleUniformTypeFloat, values: &[f32]) {
    validate_uniform_f32(count, uniform_type, values);
    let count = count as i32;
    unsafe {
        let value_ptr = values.as_ptr();
        match uniform_type {
            SimpleUniform1f => gl::Uniform1fv(location, count, value_ptr),
            SimpleUniform2f => gl::Uniform2fv(location, count, value_ptr),
            SimpleUniform3f => gl::Uniform3fv(location, count, value_ptr),
            SimpleUniform4f => gl::Uniform4fv(location, count, value_ptr)
        }
    }
}

pub fn uniform_matrix(location: i32, count: uint, uniform_type: SimpleUniformTypeMatrix, transpose: bool, values: &[f32]) {
    validate_uniform_matrix(count, uniform_type, values);
    let count = count as i32;
    let transpose = if transpose { gl::TRUE } else { gl::FALSE };
    unsafe {
        let value_ptr = values.as_ptr();
        match uniform_type {
            SimpleUniformMatrix2f => gl::UniformMatrix2fv(location, count, transpose, value_ptr),
            SimpleUniformMatrix3f => gl::UniformMatrix3fv(location, count, transpose, value_ptr),
            SimpleUniformMatrix4f => gl::UniformMatrix4fv(location, count, transpose, value_ptr),
            SimpleUniformMatrix2x3f => gl::UniformMatrix2x3fv(location, count, transpose, value_ptr),
            SimpleUniformMatrix3x2f => gl::UniformMatrix3x2fv(location, count, transpose, value_ptr),
            SimpleUniformMatrix2x4f => gl::UniformMatrix2x4fv(location, count, transpose, value_ptr),
            SimpleUniformMatrix4x2f => gl::UniformMatrix4x2fv(location, count, transpose, value_ptr),
            SimpleUniformMatrix3x4f => gl::UniformMatrix3x4fv(location, count, transpose, value_ptr),
            SimpleUniformMatrix4x3f => gl::UniformMatrix4x3fv(location, count, transpose, value_ptr),
        }
    }
}

pub fn uniform_u32(location: i32, count: uint, uniform_type: SimpleUniformTypeUint, values: &[u32]) {
    validate_uniform_u32(count, uniform_type, values);
    let count = count as i32;
    unsafe {
        let value_ptr = values.as_ptr();
        match uniform_type {
            SimpleUniform1u => gl::Uniform1uiv(location, count, value_ptr),
            SimpleUniform2u => gl::Uniform2uiv(location, count, value_ptr),
            SimpleUniform3u => gl::Uniform3uiv(location, count, value_ptr),
            SimpleUniform4u => gl::Uniform4uiv(location, count, value_ptr),
        }
    }
}

pub fn uniform_i32(location: i32, count: uint, uniform_type: SimpleUniformTypeInt, values: &[i32]) {
    validate_uniform_i32(count, uniform_type, values);
    let count = count as i32;
    unsafe {
        let value_ptr = values.as_ptr();
        match uniform_type {
            SimpleUniform1i => gl::Uniform1iv(location, count, value_ptr),
            SimpleUniform2i => gl::Uniform2iv(location, count, value_ptr),
            SimpleUniform3i => gl::Uniform3iv(location, count, value_ptr),
            SimpleUniform4i => gl::Uniform4iv(location, count, value_ptr),
        }
    }
}

fn validate_uniform_f32(count: uint, uniform_type: SimpleUniformTypeFloat, values: &[f32]) {
    let element_count = match uniform_type {
        SimpleUniform1f => 1,
        SimpleUniform2f => 2,
        SimpleUniform3f => 3,
        SimpleUniform4f => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_matrix(count: uint, uniform_type: SimpleUniformTypeMatrix, values: &[f32]) {
    let element_count = match uniform_type {
        SimpleUniformMatrix2f => 2 * 2,
        SimpleUniformMatrix3f => 3 * 3,
        SimpleUniformMatrix4f => 4 * 4,
        SimpleUniformMatrix2x3f => 2 * 3,
        SimpleUniformMatrix3x2f => 3 * 2,
        SimpleUniformMatrix2x4f => 2 * 4,
        SimpleUniformMatrix4x2f => 4 * 2,
        SimpleUniformMatrix3x4f => 3 * 4,
        SimpleUniformMatrix4x3f => 4 * 3
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_u32(count: uint, uniform_type: SimpleUniformTypeUint, values: &[u32]) {
    let element_count = match uniform_type {
        SimpleUniform1u => 1,
        SimpleUniform2u => 2,
        SimpleUniform3u => 3,
        SimpleUniform4u => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform_i32(count: uint, uniform_type: SimpleUniformTypeInt, values: &[i32]) {
    let element_count = match uniform_type {
        SimpleUniform1i => 1,
        SimpleUniform2i => 2,
        SimpleUniform3i => 3,
        SimpleUniform4i => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

fn validate_uniform<T, U: Show>(count: uint, uniform_type: U, element_count: uint, values: &[T]) {
    let expected_len = count * element_count;
    if expected_len > values.len() {
        fail!("Too small uniform value slice: {} of {} would take {} elements, but only got {}",
            count, uniform_type, expected_len, values.len());
    }
}