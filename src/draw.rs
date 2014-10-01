
use gl;
use gl::types::{GLint,GLsizei};

pub fn draw_arrays(first: u32, count: u32) {
    gl::DrawArrays(gl::TRIANGLES, first as GLint, count as GLsizei);
    check_error!();
}