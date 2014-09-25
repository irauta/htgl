
use gl;
use gl::types::{GLint,GLsizei};

use super::util::check_error;

pub fn draw_arrays(first: u32, count: u32) {
    gl::DrawArrays(gl::TRIANGLES, first as GLint, count as GLsizei);
    check_error();
}