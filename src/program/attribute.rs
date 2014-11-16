
use gl;

use super::super::util::slice_to_string;
use super::Program;
use super::super::{ShaderAttributeType,
    ShaderAttributeFloat,
    ShaderAttributeFloatVec2,
    ShaderAttributeFloatVec3,
    ShaderAttributeFloatVec4,
    ShaderAttributeFloatMat2,
    ShaderAttributeFloatMat3,
    ShaderAttributeFloatMat4,
    ShaderAttributeFloatMat2x3,
    ShaderAttributeFloatMat2x4,
    ShaderAttributeFloatMat3x2,
    ShaderAttributeFloatMat3x4,
    ShaderAttributeFloatMat4x2,
    ShaderAttributeFloatMat4x3,
    ShaderAttributeInt,
    ShaderAttributeIntVec2,
    ShaderAttributeIntVec3,
    ShaderAttributeIntVec4,
    ShaderAttributeUnsignedInt,
    ShaderAttributeUnsignedIntVec2,
    ShaderAttributeUnsignedIntVec3,
    ShaderAttributeUnsignedIntVec4};

#[deriving(Show)]
pub struct ShaderAttributeInfo {
    pub attributes: Vec<ShaderAttribute>
}

#[deriving(Show)]
pub struct ShaderAttribute {
    pub name: String,
    pub attribute_type: Option<ShaderAttributeType>,
    pub size: i32
}

pub fn make_attribute_info_vec(program: &Program) -> ShaderAttributeInfo {
    let attr_count = program.get_value(gl::ACTIVE_ATTRIBUTES);
    let max_length = program.get_value(gl::ACTIVE_ATTRIBUTE_MAX_LENGTH);
    let mut name_vec = Vec::from_elem(max_length as uint, 0u8);
    ShaderAttributeInfo { attributes: Vec::from_fn(attr_count as uint, |i| {
        unsafe {
            let mut actual_length = 0;
            let mut size = 0;
            let mut gl_type = 0;
            let name_vec_ptr = name_vec.as_mut_ptr() as *mut i8;
            gl::GetActiveAttrib(program.id, i as u32, name_vec.len() as i32, &mut actual_length, &mut size, &mut gl_type, name_vec_ptr);
            let name = slice_to_string(name_vec.slice_to_or_fail(&(actual_length as uint)));
            let attribute_type = attribute_type_from_u32(gl_type);
            ShaderAttribute {
                name: name,
                attribute_type: attribute_type,
                size: size
            }
        }
    })}
}

fn attribute_type_from_u32(gl_type: u32) -> Option<ShaderAttributeType> {
    match gl_type {
        gl::FLOAT => Some(ShaderAttributeFloat),
        gl::FLOAT_VEC2 => Some(ShaderAttributeFloatVec2),
        gl::FLOAT_VEC3 => Some(ShaderAttributeFloatVec3),
        gl::FLOAT_VEC4 => Some(ShaderAttributeFloatVec4),
        gl::FLOAT_MAT2 => Some(ShaderAttributeFloatMat2),
        gl::FLOAT_MAT3 => Some(ShaderAttributeFloatMat3),
        gl::FLOAT_MAT4 => Some(ShaderAttributeFloatMat4),
        gl::FLOAT_MAT2x3 => Some(ShaderAttributeFloatMat2x3),
        gl::FLOAT_MAT2x4 => Some(ShaderAttributeFloatMat2x4),
        gl::FLOAT_MAT3x2 => Some(ShaderAttributeFloatMat3x2),
        gl::FLOAT_MAT3x4 => Some(ShaderAttributeFloatMat3x4),
        gl::FLOAT_MAT4x2 => Some(ShaderAttributeFloatMat4x2),
        gl::FLOAT_MAT4x3 => Some(ShaderAttributeFloatMat4x3),
        gl::INT => Some(ShaderAttributeInt),
        gl::INT_VEC2 => Some(ShaderAttributeIntVec2),
        gl::INT_VEC3 => Some(ShaderAttributeIntVec3),
        gl::INT_VEC4 => Some(ShaderAttributeIntVec4),
        gl::UNSIGNED_INT => Some(ShaderAttributeUnsignedInt),
        gl::UNSIGNED_INT_VEC2 => Some(ShaderAttributeUnsignedIntVec2),
        gl::UNSIGNED_INT_VEC3 => Some(ShaderAttributeUnsignedIntVec3),
        gl::UNSIGNED_INT_VEC4 => Some(ShaderAttributeUnsignedIntVec4),
        _ => None
    }
}