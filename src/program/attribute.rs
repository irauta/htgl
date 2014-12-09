
use gl;

use super::super::util::slice_to_string;
use super::Program;

#[deriving(Show)]
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

#[deriving(Show)]
pub struct ShaderAttributeInfo {
    pub attributes: Vec<ShaderAttribute>
}

impl ShaderAttributeInfo {
    pub fn get_attribute(&self, name: &str) -> Option<&ShaderAttribute> {
        for attribute in self.attributes.iter() {
            if attribute.name[] == name {
                return Some(attribute);
            }
        }
        None
    }
}

#[deriving(Show)]
pub struct ShaderAttribute {
    pub name: String,
    pub location: i32,
    pub attribute_type: Option<ShaderAttributeType>,
    pub size: i32
}

pub fn make_attribute_info_vec(program: &Program) -> ShaderAttributeInfo {
    let attr_count = program.get_value(gl::ACTIVE_ATTRIBUTES);
    let max_length = program.get_value(gl::ACTIVE_ATTRIBUTE_MAX_LENGTH);
    let mut name_vec = Vec::from_elem(max_length as uint, 0u8);
    ShaderAttributeInfo { attributes: Vec::from_fn(attr_count as uint, |i| {
        let mut actual_length = 0;
        let mut size = 0;
        let mut gl_type = 0;
        let name_vec_ptr = name_vec.as_mut_ptr() as *mut i8;
        unsafe {
            gl::GetActiveAttrib(program.id, i as u32, name_vec.len() as i32, &mut actual_length, &mut size, &mut gl_type, name_vec_ptr);
        }
        let name = slice_to_string(name_vec.slice_to_or_fail(&(actual_length as uint)));
        let attribute_type = attribute_type_from_u32(gl_type);
        let location = program.get_attribute_location(name[]);
        ShaderAttribute {
            name: name,
            location: location,
            attribute_type: attribute_type,
            size: size
        }
    })}
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