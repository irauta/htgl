
extern crate glfw;

extern crate mog;

use glfw::Context;

use mog::{AttributeFloat,AttributeUnsignedByte,ClearColor,DepthTest,VertexShader,FragmentShader};

#[allow(dead_code)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[allow(dead_code)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[allow(dead_code)]
struct Vertex {
    position: Vec3,
    color: Rgba
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8, a: u8) -> Vertex {
        Vertex { position: Vec3 { x: x, y: y, z: z }, color: Rgba { r: r, g: g, b: b, a: a } }
    }
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    mog::load_with(|s| window.get_proc_address(s));

    let mut ctx = mog::Context::new();
    ctx.set_option(ClearColor(1f32, 1f32, 1f32, 1f32));
    ctx.set_option(DepthTest(false));
    let vbo = ctx.new_vertex_buffer();
    let vertices = [
        Vertex::new(-0.5f32, -0.5f32, 0f32, 255, 0, 0, 0),
        Vertex::new(0.5f32, -0.5f32, 0f32, 0, 255, 0, 0),
        Vertex::new(0f32, 0.5f32, 0f32, 0, 0, 255, 0)
        ];
    ctx.vertex_data(&vbo, &vertices);
    let vao = ctx.new_vertex_array_simple([(3, AttributeFloat, false), (4, AttributeUnsignedByte, true)], vbo, None);

    let vs = ctx.new_shader(VertexShader, "");
    let fs = ctx.new_shader(FragmentShader, "");
    let program = ctx.new_program(&[vs, fs]);
    ctx.use_program(&program);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }

        ctx.clear();
        ctx.draw_arrays(0, 3);

        window.swap_buffers();
    }
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
