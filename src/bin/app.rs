
extern crate mog;

fn main() {
    let mut ctx = mog::Context::new();
    let vbo = ctx.new_vertex_buffer();
    ctx.vertex_data(&vbo, [0u32]);
}