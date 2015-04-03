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

use std::iter::repeat;
use std::ptr::null_mut;
use std::fmt::Debug;
use std::ffi::CString;

use gl;
use gl::types::GLenum;

use super::Program;

//! This module handles management of uniform variables in OpenGL program objects. This includes
//! being able to set uniform variables directly, but also querying program introspection info on
//! the uniforms and uniform blocks the program has. What this module does not do, is to create
//! uniform buffer contents for you, just the information that is needed to do so. (Also see the
//! `info` module and the uniform block offset alignment and the uniform block maximum size
//! values.)

/// A helper enum to be used when setting a uniform's value directly (not through a uniform
/// buffer). Use it to specify single float values or float vector values. (Or arrays of them.)
#[derive(Copy,Debug)]
pub enum SimpleUniformTypeFloat {
    Uniform1f,
    Uniform2f,
    Uniform3f,
    Uniform4f
}

/// A helper enum to be used when setting a uniform's value directly (not through a uniform
/// buffer). Use it to specify matrices of certain dimensions or arrays of such matrices.
#[derive(Copy,Debug)]
pub enum SimpleUniformTypeMatrix {
    Matrix2f,
    Matrix3f,
    Matrix4f,
    Matrix2x3f,
    Matrix3x2f,
    Matrix2x4f,
    Matrix4x2f,
    Matrix3x4f,
    Matrix4x3f
}

/// A helper enum to be used when setting a uniform's value directly (not through a uniform
/// buffer). Use it to specify single i32 values or i32 vector values. (Or arrays of them.)
#[derive(Copy,Debug)]
pub enum SimpleUniformTypeI32 {
    Uniform1i,
    Uniform2i,
    Uniform3i,
    Uniform4i
}

/// A helper enum to be used when setting a uniform's value directly (not through a uniform
/// buffer). Use it to specify single u32 values or u32 vector values. (Or arrays of them.)
#[derive(Copy,Debug)]
pub enum SimpleUniformTypeU32 {
    Uniform1u,
    Uniform2u,
    Uniform3u,
    Uniform4u
}

/// Enum for different recognized uniform data types. Note that there is also a variant that
/// handles the types that are not recognized by this library. See glGetActiveUniformsiv for
/// the official list of values.
#[derive(Copy,Debug)]
pub enum UniformType {
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    Int,
    IntVec2,
    IntVec3,
    IntVec4,
    UnsignedInt,
    UnsignedIntVec2,
    UnsignedIntVec3,
    UnsignedIntVec4,
    Bool,
    BoolVec2,
    BoolVec3,
    BoolVec4,
    FloatMat2,
    FloatMat3,
    FloatMat4,
    FloatMat2x3,
    FloatMat2x4,
    FloatMat3x2,
    FloatMat3x4,
    FloatMat4x2,
    FloatMat4x3,
    Sampler1d,
    Sampler2d,
    Sampler3d,
    SamplerCube,
    Sampler1dShadow,
    Sampler2dShadow,
    Sampler1dArray,
    Sampler2dArray,
    Sampler1dArrayShadow,
    Sampler2dArrayShadow,
    Sampler2dMultisample,
    Sampler2dMultisampleArray,
    SamplerCubeShadow,
    SamplerBuffer,
    Sampler2dRect,
    Sampler2dRectShadow,
    IntSampler1d,
    IntSampler2d,
    IntSampler3d,
    IntSamplerCube,
    IntSampler1dArray,
    IntSampler2dArray,
    IntSampler2dMultisample,
    IntSampler2dMultisampleArray,
    IntSamplerBuffer,
    IntSampler2dRect,
    UnsignedIntSampler1d,
    UnsignedIntSampler2d,
    UnsignedIntSampler3d,
    UnsignedIntSamplerCube,
    UnsignedIntSampler1dArray,
    UnsignedIntSampler2dArray,
    UnsignedIntSampler2dMultisample,
    UnsignedIntSampler2dMultisampleArray,
    UnsignedIntSamplerBuffer,
    UnsignedIntSampler2dRect,
    UnrecognizedType(u32)
}

/// Helper struct containing all the information a GL uniform has. This is only an intermediary
/// to use when gathering the uniform information, before passing it to user in a nicer form.
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

/// Top-level result structure for program's uniform introspection info.
#[derive(Debug)]
pub struct UniformInfo {
    /// Global uniforms, not in interface blocks.
    pub globals: Vec<Uniform>,
    /// Interface block definitions, may contain several uniforms themselves.
    pub blocks: Vec<InterfaceBlock>
}

impl UniformInfo {
    /// Convenience method that seeks a global uniform by name and returns a refernce to it if
    /// found.
    pub fn get_global_uniform(&self, name: &str) -> Option<&Uniform> {
        for uniform in self.globals.iter() {
            if uniform.name == name {
                return Some(uniform);
            }
        }
        None
    }

    /// Convenience method that seeks an interface block by name.
    pub fn get_block(&self, name: &str) -> Option<&InterfaceBlock> {
        for block in self.blocks.iter() {
            if block.name == name {
                return Some(block);
            }
        }
        None
    }

    /// Convenience method that seeks a uniform by name from an interface block with specific name.
    pub fn get_block_uniform(&self, block_name: &str, uniform_name: &str) -> Option<&BlockUniform> {
        if let Some(block) = self.get_block(block_name) {
            return block.get_uniform(uniform_name);
        }
        None
    }
}

/// A uniform not in a block. A "global" uniform.
#[derive(Debug)]
pub struct Uniform {
    /// Name of the uniform.
    pub name: String,
    /// Location of the uniform, use this when setting value of the uniform, not the index in the
    /// vector that describes the uniforms; they may not be the same.
    pub location: i32,
    /// Data type of the uniform.
    pub uniform_type: UniformType,
    /// How many instances of the type this uniform contains. Length of an array so to speak.
    pub size: i32
}

impl Uniform {
    fn new(gl_uniform: GlUniform, location: i32) -> Uniform {
        Uniform {
            name: gl_uniform.name,
            location: location,
            uniform_type: uniform_type_from_u32(gl_uniform.uniform_type as u32),
            size: gl_uniform.size
        }
    }
}

/// Description of an interface block.
#[derive(Debug)]
pub struct InterfaceBlock {
    /// Name of the block.
    pub name: String,
    /// Index of the block. Use this as the location/index, not the index in the vector this
    /// struct is in!
    pub index: u32,
    /// See GL_UNIFORM_BLOCK_DATA_SIZE
    pub data_size: i32,
    /// The uniforms contained by this block.
    pub uniforms: Vec<BlockUniform>
}

impl InterfaceBlock {
    pub fn get_uniform(&self, name: &str) -> Option<&BlockUniform> {
        for uniform in self.uniforms.iter() {
            if uniform.name == name {
                return Some(uniform);
            }
        }
        None
    }
}

/// A uniform contained within a block.
/// TODO: Missing info whether a matrix uniform is row major.
#[derive(Debug)]
pub struct BlockUniform {
    /// Name of the uniform.
    pub name: String,
    /// Data type of the uniform.
    pub uniform_type: UniformType,
    /// How long is the array of the uniforms (if the uniform is an array uniform).
    pub size: i32,
    /// How many bytes from the beginning of the block this uniform is. See GL_UNIFORM_OFFSET.
    pub offset: i32,
    /// For an array uniform, the distance between each value in the array.
    /// See GL_UNIFORM_ARRAY_STRIDE.
    pub array_stride: i32,
    /// Distance between rows/cols of a matrix uniform. See GL_UNIFORM_MATRIX_STRIDE.
    pub matrix_stride: i32,
}

impl BlockUniform {
    fn new(gl_uniform: GlUniform) -> BlockUniform {
        BlockUniform {
            name: gl_uniform.name,
            uniform_type: uniform_type_from_u32(gl_uniform.uniform_type as u32),
            size: gl_uniform.size,
            offset: gl_uniform.offset,
            array_stride: gl_uniform.array_stride,
            matrix_stride: gl_uniform.matrix_stride
        }
    }
}

/// Compiles available uniform information into a big struct.
pub fn make_uniform_info(program: &Program) -> UniformInfo {
    let gl_uniforms = make_gl_uniform_info_vec(program);
    let mut globals = Vec::new();
    let mut blocks = make_uniform_block_info_vec(program);
    for gl_uniform in gl_uniforms.into_iter() {
        if gl_uniform.block_index < 0 {
            let location = program.get_uniform_location(&gl_uniform.name[..]);
            globals.push(Uniform::new(gl_uniform, location));
        }
        else {
            let index = gl_uniform.block_index as usize;
            blocks[index].uniforms.push(BlockUniform::new(gl_uniform));
        }
    }
    UniformInfo {
        globals: globals,
        blocks: blocks
    }
}

/// Builds a vector of GlUniform that represents global uniforms of a program.
fn make_gl_uniform_info_vec(program: &Program) -> Vec<GlUniform> {
    let count = program.get_value(gl::ACTIVE_UNIFORMS) as usize;
    if count == 0 {
        return Vec::new();
    }
    //let mut info_vec = Vec::with_capacity(count);
    let mut intvalues = repeat(0).take(count).collect();
    let indices = (0..count as u32).collect();
    fill_uniform_info_vec(program.id, &indices, gl::UNIFORM_NAME_LENGTH, &mut intvalues);
    //for (index, expected_len) in intvalues.iter().enumerate() {
    //    info_vec.push(GlUniform::new(uniform_name(program.id, index as u32, *expected_len as u32)));
    //}
    let mut info_vec: Vec<GlUniform> = intvalues.iter().enumerate()
        .map(|(index, expected_len)| GlUniform::new(uniform_name(program.id, index as u32, *expected_len as u32)))
        .collect();
    {
        let mut fill_info = |property, info_fn: &mut Fn(&mut GlUniform, i32)| {
            fill_uniform_info_vec(program.id, &indices, property, &mut intvalues);
            for (info, value) in info_vec.iter_mut().zip(intvalues.iter()) {
                info_fn(info, *value);
            }
        };
        fill_info(gl::UNIFORM_SIZE, &mut|info, value| info.size = value);
        fill_info(gl::UNIFORM_TYPE, &mut|info, value| info.uniform_type = value);
        fill_info(gl::UNIFORM_OFFSET, &mut|info, value| info.offset = value);
        fill_info(gl::UNIFORM_BLOCK_INDEX, &mut|info, value| info.block_index = value);
        fill_info(gl::UNIFORM_ARRAY_STRIDE, &mut|info, value| info.array_stride = value);
        fill_info(gl::UNIFORM_MATRIX_STRIDE, &mut|info, value| info.matrix_stride = value);
    }
    info_vec
}

/// Builds a vector of GlUniform that represents uniform blocks of a program.
fn make_uniform_block_info_vec(program: &Program) -> Vec<InterfaceBlock> {
    let count = program.get_value(gl::ACTIVE_UNIFORM_BLOCKS);
    if count == 0 {
        return Vec::new();
    }
    let mut info_vec = Vec::with_capacity(count as usize);
    for index in 0..count as u32 {
        let expected_len = get_block_info(program.id, index, gl::UNIFORM_BLOCK_NAME_LENGTH) as u32;
        let data_size = get_block_info(program.id, index, gl::UNIFORM_BLOCK_DATA_SIZE);
        let name = block_name(program.id, index, expected_len);
        let index = get_uniform_block_index(program.id, &name[..]);
        info_vec.push(InterfaceBlock {
            index: index,
            name: name,
            data_size: data_size,
            uniforms: Vec::new()
        });
    }
    info_vec
}

/// Gets a single uniform-describing attribute for multiple uniforms as identified by the indices.
fn fill_uniform_info_vec(program_id: u32, indices: &Vec<u32>, property: GLenum, intvalues: &mut Vec<i32>) {
    unsafe {
        gl::GetActiveUniformsiv(program_id, indices.len() as i32, indices.as_ptr(), property, intvalues.as_mut_ptr());
        check_error!();
    }
}

/// Gets a value related to a uniform block.
fn get_block_info(program_id: u32, block_index: u32, property: GLenum) -> i32 {
    unsafe {
        let mut value = 0;
        gl::GetActiveUniformBlockiv(program_id, block_index, property, &mut value);
        check_error!();
        value
    }
}

/// Finds out what a uniform's name is.
fn uniform_name(program_id: u32, index: u32, expected_len: u32) -> String {
    let mut name_vec: Vec<u8> = repeat(0u8).take(expected_len as usize).collect();
    unsafe {
        let name_ptr = name_vec.as_mut_slice().as_mut_ptr() as *mut i8;
        gl::GetActiveUniformName(program_id, index, name_vec.len() as i32, null_mut(), name_ptr);
        check_error!();
    }
    name_vec.pop(); // Remove the null byte from end
    String::from_utf8(name_vec).unwrap()
}

/// Finds out what an interface block's name is.
fn block_name(program_id: u32, index: u32, expected_len: u32) -> String {
    let mut name_vec: Vec<u8> = repeat(0u8).take(expected_len as usize).collect();
    unsafe {
        let name_ptr = name_vec.as_mut_slice().as_mut_ptr() as *mut i8;
        gl::GetActiveUniformBlockName(program_id, index, name_vec.len() as i32, null_mut(), name_ptr);
        check_error!();
    }
    name_vec.pop(); // Remove the null byte from end
    String::from_utf8(name_vec).unwrap()
}

fn get_uniform_block_index(program_id: u32, name: &str) -> u32 {
    let c_name = CString::new(name).unwrap();
    unsafe {
        let index = gl::GetUniformBlockIndex(program_id, c_name.as_ptr());
        check_error!();
        index
    }
}

/// Set uniform values of type f32. (Single values, 2D, 3D, 4D vectors, or arrays of them.)
pub fn uniform_f32(location: i32, count: usize, uniform_type: SimpleUniformTypeFloat, values: &[f32]) {
    validate_uniform_f32(count, uniform_type, values);
    let count = count as i32;
    unsafe {
        let value_ptr = values.as_ptr();
        match uniform_type {
            SimpleUniformTypeFloat::Uniform1f => gl::Uniform1fv(location, count, value_ptr),
            SimpleUniformTypeFloat::Uniform2f => gl::Uniform2fv(location, count, value_ptr),
            SimpleUniformTypeFloat::Uniform3f => gl::Uniform3fv(location, count, value_ptr),
            SimpleUniformTypeFloat::Uniform4f => gl::Uniform4fv(location, count, value_ptr)
        }
    }
}

/// Set uniform matrix values.
pub fn uniform_matrix(location: i32, count: usize, uniform_type: SimpleUniformTypeMatrix, transpose: bool, values: &[f32]) {
    validate_uniform_matrix(count, uniform_type, values);
    let count = count as i32;
    let transpose = if transpose { gl::TRUE } else { gl::FALSE };
    unsafe {
        let value_ptr = values.as_ptr();
        match uniform_type {
            SimpleUniformTypeMatrix::Matrix2f => gl::UniformMatrix2fv(location, count, transpose, value_ptr),
            SimpleUniformTypeMatrix::Matrix3f => gl::UniformMatrix3fv(location, count, transpose, value_ptr),
            SimpleUniformTypeMatrix::Matrix4f => gl::UniformMatrix4fv(location, count, transpose, value_ptr),
            SimpleUniformTypeMatrix::Matrix2x3f => gl::UniformMatrix2x3fv(location, count, transpose, value_ptr),
            SimpleUniformTypeMatrix::Matrix3x2f => gl::UniformMatrix3x2fv(location, count, transpose, value_ptr),
            SimpleUniformTypeMatrix::Matrix2x4f => gl::UniformMatrix2x4fv(location, count, transpose, value_ptr),
            SimpleUniformTypeMatrix::Matrix4x2f => gl::UniformMatrix4x2fv(location, count, transpose, value_ptr),
            SimpleUniformTypeMatrix::Matrix3x4f => gl::UniformMatrix3x4fv(location, count, transpose, value_ptr),
            SimpleUniformTypeMatrix::Matrix4x3f => gl::UniformMatrix4x3fv(location, count, transpose, value_ptr),
        }
    }
}

/// Set uniform values of type u32. (Single values, 2D, 3D, 4D vectors, or arrays of them.)
pub fn uniform_u32(location: i32, count: usize, uniform_type: SimpleUniformTypeU32, values: &[u32]) {
    validate_uniform_u32(count, uniform_type, values);
    let count = count as i32;
    unsafe {
        let value_ptr = values.as_ptr();
        match uniform_type {
            SimpleUniformTypeU32::Uniform1u => gl::Uniform1uiv(location, count, value_ptr),
            SimpleUniformTypeU32::Uniform2u => gl::Uniform2uiv(location, count, value_ptr),
            SimpleUniformTypeU32::Uniform3u => gl::Uniform3uiv(location, count, value_ptr),
            SimpleUniformTypeU32::Uniform4u => gl::Uniform4uiv(location, count, value_ptr),
        }
    }
}

/// Set uniform values of type i32. (Single values, 2D, 3D, 4D vectors, or arrays of them.)
pub fn uniform_i32(location: i32, count: usize, uniform_type: SimpleUniformTypeI32, values: &[i32]) {
    validate_uniform_i32(count, uniform_type, values);
    let count = count as i32;
    unsafe {
        let value_ptr = values.as_ptr();
        match uniform_type {
            SimpleUniformTypeI32::Uniform1i => gl::Uniform1iv(location, count, value_ptr),
            SimpleUniformTypeI32::Uniform2i => gl::Uniform2iv(location, count, value_ptr),
            SimpleUniformTypeI32::Uniform3i => gl::Uniform3iv(location, count, value_ptr),
            SimpleUniformTypeI32::Uniform4i => gl::Uniform4iv(location, count, value_ptr),
        }
    }
}

/// Check that there's enough values in the slice to set `count` uniforms of given type.
fn validate_uniform_f32(count: usize, uniform_type: SimpleUniformTypeFloat, values: &[f32]) {
    let element_count = match uniform_type {
        SimpleUniformTypeFloat::Uniform1f => 1,
        SimpleUniformTypeFloat::Uniform2f => 2,
        SimpleUniformTypeFloat::Uniform3f => 3,
        SimpleUniformTypeFloat::Uniform4f => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

/// Check that there's enough values in the slice to set `count` uniforms of given type.
fn validate_uniform_matrix(count: usize, uniform_type: SimpleUniformTypeMatrix, values: &[f32]) {
    let element_count = match uniform_type {
        SimpleUniformTypeMatrix::Matrix2f => 2 * 2,
        SimpleUniformTypeMatrix::Matrix3f => 3 * 3,
        SimpleUniformTypeMatrix::Matrix4f => 4 * 4,
        SimpleUniformTypeMatrix::Matrix2x3f => 2 * 3,
        SimpleUniformTypeMatrix::Matrix3x2f => 3 * 2,
        SimpleUniformTypeMatrix::Matrix2x4f => 2 * 4,
        SimpleUniformTypeMatrix::Matrix4x2f => 4 * 2,
        SimpleUniformTypeMatrix::Matrix3x4f => 3 * 4,
        SimpleUniformTypeMatrix::Matrix4x3f => 4 * 3
    };
    validate_uniform(count, uniform_type, element_count, values);
}

/// Check that there's enough values in the slice to set `count` uniforms of given type.
fn validate_uniform_u32(count: usize, uniform_type: SimpleUniformTypeU32, values: &[u32]) {
    let element_count = match uniform_type {
        SimpleUniformTypeU32::Uniform1u => 1,
        SimpleUniformTypeU32::Uniform2u => 2,
        SimpleUniformTypeU32::Uniform3u => 3,
        SimpleUniformTypeU32::Uniform4u => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

/// Check that there's enough values in the slice to set `count` uniforms of given type.
fn validate_uniform_i32(count: usize, uniform_type: SimpleUniformTypeI32, values: &[i32]) {
    let element_count = match uniform_type {
        SimpleUniformTypeI32::Uniform1i => 1,
        SimpleUniformTypeI32::Uniform2i => 2,
        SimpleUniformTypeI32::Uniform3i => 3,
        SimpleUniformTypeI32::Uniform4i => 4
    };
    validate_uniform(count, uniform_type, element_count, values);
}

/// Common parts of the uniform value validation routine.
fn validate_uniform<T, U: Debug>(count: usize, uniform_type: U, element_count: usize, values: &[T]) {
    let expected_len = count * element_count;
    if expected_len > values.len() {
        panic!("Too small uniform value slice: {} of {:?} would take {} elements, but only got {}",
            count, uniform_type, expected_len, values.len());
    }
}

fn uniform_type_from_u32(gl_type: u32) -> UniformType {
    match gl_type {
        gl::FLOAT => UniformType::Float,
        gl::FLOAT_VEC2 => UniformType::FloatVec2,
        gl::FLOAT_VEC3 => UniformType::FloatVec3,
        gl::FLOAT_VEC4 => UniformType::FloatVec4,
        gl::INT => UniformType::Int,
        gl::INT_VEC2 => UniformType::IntVec2,
        gl::INT_VEC3 => UniformType::IntVec3,
        gl::INT_VEC4 => UniformType::IntVec4,
        gl::UNSIGNED_INT => UniformType::UnsignedInt,
        gl::UNSIGNED_INT_VEC2 => UniformType::UnsignedIntVec2,
        gl::UNSIGNED_INT_VEC3 => UniformType::UnsignedIntVec3,
        gl::UNSIGNED_INT_VEC4 => UniformType::UnsignedIntVec4,
        gl::BOOL => UniformType::Bool,
        gl::BOOL_VEC2 => UniformType::BoolVec2,
        gl::BOOL_VEC3 => UniformType::BoolVec3,
        gl::BOOL_VEC4 => UniformType::BoolVec4,
        gl::FLOAT_MAT2 => UniformType::FloatMat2,
        gl::FLOAT_MAT3 => UniformType::FloatMat3,
        gl::FLOAT_MAT4 => UniformType::FloatMat4,
        gl::FLOAT_MAT2x3 => UniformType::FloatMat2x3,
        gl::FLOAT_MAT2x4 => UniformType::FloatMat2x4,
        gl::FLOAT_MAT3x2 => UniformType::FloatMat3x2,
        gl::FLOAT_MAT3x4 => UniformType::FloatMat3x4,
        gl::FLOAT_MAT4x2 => UniformType::FloatMat4x2,
        gl::FLOAT_MAT4x3 => UniformType::FloatMat4x3,
        gl::SAMPLER_1D => UniformType::Sampler1d,
        gl::SAMPLER_2D => UniformType::Sampler2d,
        gl::SAMPLER_3D => UniformType::Sampler3d,
        gl::SAMPLER_CUBE => UniformType::SamplerCube,
        gl::SAMPLER_1D_SHADOW => UniformType::Sampler1dShadow,
        gl::SAMPLER_2D_SHADOW => UniformType::Sampler2dShadow,
        gl::SAMPLER_1D_ARRAY => UniformType::Sampler1dArray,
        gl::SAMPLER_2D_ARRAY => UniformType::Sampler2dArray,
        gl::SAMPLER_1D_ARRAY_SHADOW => UniformType::Sampler1dArrayShadow,
        gl::SAMPLER_2D_ARRAY_SHADOW => UniformType::Sampler2dArrayShadow,
        gl::SAMPLER_2D_MULTISAMPLE => UniformType::Sampler2dMultisample,
        gl::SAMPLER_2D_MULTISAMPLE_ARRAY => UniformType::Sampler2dMultisampleArray,
        gl::SAMPLER_CUBE_SHADOW => UniformType::SamplerCubeShadow,
        gl::SAMPLER_BUFFER => UniformType::SamplerBuffer,
        gl::SAMPLER_2D_RECT => UniformType::Sampler2dRect,
        gl::SAMPLER_2D_RECT_SHADOW => UniformType::Sampler2dRectShadow,
        gl::INT_SAMPLER_1D => UniformType::IntSampler1d,
        gl::INT_SAMPLER_2D => UniformType::IntSampler2d,
        gl::INT_SAMPLER_3D => UniformType::IntSampler3d,
        gl::INT_SAMPLER_CUBE => UniformType::IntSamplerCube,
        gl::INT_SAMPLER_1D_ARRAY => UniformType::IntSampler1dArray,
        gl::INT_SAMPLER_2D_ARRAY => UniformType::IntSampler2dArray,
        gl::INT_SAMPLER_2D_MULTISAMPLE => UniformType::IntSampler2dMultisample,
        gl::INT_SAMPLER_2D_MULTISAMPLE_ARRAY => UniformType::IntSampler2dMultisampleArray,
        gl::INT_SAMPLER_BUFFER => UniformType::IntSamplerBuffer,
        gl::INT_SAMPLER_2D_RECT => UniformType::IntSampler2dRect,
        gl::UNSIGNED_INT_SAMPLER_1D => UniformType::UnsignedIntSampler1d,
        gl::UNSIGNED_INT_SAMPLER_2D => UniformType::UnsignedIntSampler2d,
        gl::UNSIGNED_INT_SAMPLER_3D => UniformType::UnsignedIntSampler3d,
        gl::UNSIGNED_INT_SAMPLER_CUBE => UniformType::UnsignedIntSamplerCube,
        gl::UNSIGNED_INT_SAMPLER_1D_ARRAY => UniformType::UnsignedIntSampler1dArray,
        gl::UNSIGNED_INT_SAMPLER_2D_ARRAY => UniformType::UnsignedIntSampler2dArray,
        gl::UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE => UniformType::UnsignedIntSampler2dMultisample,
        gl::UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY => UniformType::UnsignedIntSampler2dMultisampleArray,
        gl::UNSIGNED_INT_SAMPLER_BUFFER => UniformType::UnsignedIntSamplerBuffer,
        gl::UNSIGNED_INT_SAMPLER_2D_RECT => UniformType::UnsignedIntSampler2dRect,
        _ => UniformType::UnrecognizedType(gl_type)
    }
}