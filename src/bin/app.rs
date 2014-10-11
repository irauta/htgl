
#![feature(slicing_syntax,if_let)]

extern crate glfw;

extern crate mog;

use glfw::Context;

use mog::{AttributeFloat,AttributeUnsignedByte,ClearColor,DepthTest,CullingEnabled,VertexShader,FragmentShader,Triangles,Uniform1f};

#[allow(dead_code)]
#[repr(packed)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[allow(dead_code)]
#[repr(packed)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[allow(dead_code)]
#[repr(packed)]
struct Vertex {
    position: Vec3,
    color: Rgba
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8, a: u8) -> Vertex {
        Vertex { position: Vec3 { x: x, y: y, z: z }, color: Rgba { r: r, g: g, b: b, a: a } }
    }
}

static VS_SOURCE: &'static str = "
#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

uniform float scale;

out vec4 v_color;

void main() {
    gl_Position.xyz = position * scale;
    gl_Position.w = 1.0;
    v_color = color;
}
";

static FS_SOURCE: &'static str = "
#version 330 core

in vec4 v_color;
out vec3 color;

void main() {
    color = v_color.rgb;
}
";

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::ContextVersion(3, 3));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    // Create a windowed mode window and its OpenGL context
    let (window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    mog::load_with(|s| window.get_proc_address(s));

    let mut ctx = mog::Context::new();
    ctx.renderer().set_option(ClearColor(1f32, 1f32, 1f32, 1f32));
    ctx.renderer().set_option(DepthTest(false));
    ctx.renderer().set_option(CullingEnabled(true));
    let vbo = ctx.new_vertex_buffer();
    let vertices = [
        Vertex::new(-0.5f32, -0.5f32, 0f32, 255, 0, 0, 0),
        Vertex::new(0.5f32, -0.5f32, 0f32, 0, 255, 0, 0),
        Vertex::new(0f32, 0.5f32, 0f32, 0, 0, 255, 0),
        ];
    ctx.edit_vertex_buffer(&vbo).data(&vertices);
    let ibo = ctx.new_index_buffer();
    let vao = ctx.new_vertex_array_simple([(3, AttributeFloat, false), (4, AttributeUnsignedByte, true)], vbo, Some(ibo));
    if let Some(mut editor) = ctx.edit_index_buffer(&vao) {
        let indices = [0, 1, 2];
        editor.data_u16(&indices);
    }
    let vs = ctx.new_shader(VertexShader, VS_SOURCE);
    let fs = ctx.new_shader(FragmentShader, FS_SOURCE);
    let program = ctx.new_program(&[vs, fs]);

    {
        let program_editor = ctx.edit_program(&program);
        let scale_location = program_editor.get_uniform_location("scale");
        program_editor.uniform_f32(scale_location, 1, Uniform1f, &[1.5]);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }

        let mut renderer = ctx.renderer();
        renderer.clear();
        renderer.use_vertex_array(&vao);
        renderer.use_program(&program);
        renderer.draw_elements_u16(Triangles, 3, 0);

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
