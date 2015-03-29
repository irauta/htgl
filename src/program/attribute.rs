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

use gl;

use super::super::util::slice_to_string;
use super::Program;

/// See the `type` argument of glGetActiveAttrib (the sixth one) for the set of values this enum's
/// variants correspond to.
#[derive(Debug)]
pub enum ShaderAttributeType {
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    FloatMat2,
    FloatMat3,
    FloatMat4,
    FloatMat2x3,
    FloatMat2x4,
    FloatMat3x2,
    FloatMat3x4,
    FloatMat4x2,
    FloatMat4x3,
    Int,
    IntVec2,
    IntVec3,
    IntVec4,
    UnsignedInt,
    UnsignedIntVec2,
    UnsignedIntVec3,
    UnsignedIntVec4
}

/// Contains information on shader program's (vertex) attributes.
#[derive(Debug)]
pub struct ShaderAttributeInfo {
    /// List of attributes.
    pub attributes: Vec<ShaderAttribute>
}

impl ShaderAttributeInfo {
    /// A convenience method to find an attribute by name. Not particularly optimized. It might be
    /// a good idea to only do one lookup by name and use the integer indices, borrows, or
    /// something similar from there on.
    pub fn get_attribute(&self, name: &str) -> Option<&ShaderAttribute> {
        for attribute in self.attributes.iter() {
            if attribute.name == name {
                return Some(attribute);
            }
        }
        None
    }
}

/// Describes an (active) attribute of a shader program.
#[derive(Debug)]
pub struct ShaderAttribute {
    /// Name of the attribute
    pub name: String,
    /// Index of the attribute
    pub location: i32,
    /// Data type of the attribute
    pub attribute_type: Option<ShaderAttributeType>,
    pub size: i32
}

pub fn make_attribute_info_vec(program: &Program) -> ShaderAttributeInfo {
    let attr_count = program.get_value(gl::ACTIVE_ATTRIBUTES);
    let max_length = program.get_value(gl::ACTIVE_ATTRIBUTE_MAX_LENGTH);
    let mut name_vec: Vec<u8> = repeat(0u8).take(max_length as usize).collect();
    ShaderAttributeInfo { attributes: (0..attr_count as usize).map(|i| {
        let mut actual_length = 0;
        let mut size = 0;
        let mut gl_type = 0;
        let name_vec_ptr = name_vec.as_mut_ptr() as *mut i8;
        unsafe {
            gl::GetActiveAttrib(program.id, i as u32, name_vec.len() as i32, &mut actual_length, &mut size, &mut gl_type, name_vec_ptr);
        }
        let name = slice_to_string(&name_vec[0..actual_length as usize]);
        let attribute_type = attribute_type_from_u32(gl_type);
        let location = program.get_attribute_location(&name[..]);
        ShaderAttribute {
            name: name,
            location: location,
            attribute_type: attribute_type,
            size: size
        }
    }).collect()}
}

fn attribute_type_from_u32(gl_type: u32) -> Option<ShaderAttributeType> {
    match gl_type {
        gl::FLOAT => Some(ShaderAttributeType::Float),
        gl::FLOAT_VEC2 => Some(ShaderAttributeType::FloatVec2),
        gl::FLOAT_VEC3 => Some(ShaderAttributeType::FloatVec3),
        gl::FLOAT_VEC4 => Some(ShaderAttributeType::FloatVec4),
        gl::FLOAT_MAT2 => Some(ShaderAttributeType::FloatMat2),
        gl::FLOAT_MAT3 => Some(ShaderAttributeType::FloatMat3),
        gl::FLOAT_MAT4 => Some(ShaderAttributeType::FloatMat4),
        gl::FLOAT_MAT2x3 => Some(ShaderAttributeType::FloatMat2x3),
        gl::FLOAT_MAT2x4 => Some(ShaderAttributeType::FloatMat2x4),
        gl::FLOAT_MAT3x2 => Some(ShaderAttributeType::FloatMat3x2),
        gl::FLOAT_MAT3x4 => Some(ShaderAttributeType::FloatMat3x4),
        gl::FLOAT_MAT4x2 => Some(ShaderAttributeType::FloatMat4x2),
        gl::FLOAT_MAT4x3 => Some(ShaderAttributeType::FloatMat4x3),
        gl::INT => Some(ShaderAttributeType::Int),
        gl::INT_VEC2 => Some(ShaderAttributeType::IntVec2),
        gl::INT_VEC3 => Some(ShaderAttributeType::IntVec3),
        gl::INT_VEC4 => Some(ShaderAttributeType::IntVec4),
        gl::UNSIGNED_INT => Some(ShaderAttributeType::UnsignedInt),
        gl::UNSIGNED_INT_VEC2 => Some(ShaderAttributeType::UnsignedIntVec2),
        gl::UNSIGNED_INT_VEC3 => Some(ShaderAttributeType::UnsignedIntVec3),
        gl::UNSIGNED_INT_VEC4 => Some(ShaderAttributeType::UnsignedIntVec4),
        _ => None
    }
}