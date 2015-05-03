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

extern crate glfw;

extern crate htgl;

use glfw::Context;

use htgl::{VertexAttributeType,
    RenderOption,
    ShaderType,
    PrimitiveMode,
    SimpleUniformTypeFloat};

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
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    htgl::load_with(|s| window.get_proc_address(s));

    let mut ctx = htgl::Context::new();
    println!("{:?}", ctx.get_info());
    ctx.renderer().set_option(RenderOption::ClearColor(1f32, 1f32, 1f32, 1f32));
    ctx.renderer().set_option(RenderOption::DepthTest(false));
    ctx.renderer().set_option(RenderOption::CullingEnabled(true));
    let vbo = ctx.new_buffer();
    let vertices = [
        Vertex::new(-0.5f32, -0.5f32, 0f32, 255, 0, 0, 0),
        Vertex::new(0.5f32, -0.5f32, 0f32, 0, 255, 0, 0),
        Vertex::new(0f32, 0.5f32, 0f32, 0, 0, 255, 0),
        ];
    ctx.edit_vertex_buffer(&vbo).data(&vertices);
    let ibo = ctx.new_buffer();
    let vao = ctx.new_vertex_array_simple(&[(3, VertexAttributeType::Float, false), (4, VertexAttributeType::UnsignedByte, true)], vbo, Some(ibo));
    if let Some(mut editor) = ctx.edit_index_buffer(&vao) {
        let indices = [0u16, 1u16, 2u16];
        editor.data(&indices);
    }
    let vs = ctx.new_shader(ShaderType::VertexShader, VS_SOURCE);
    if !ctx.shader_info(&vs).get_compile_status() {
        panic!(ctx.shader_info(&vs).get_info_log())
    }
    let fs = ctx.new_shader(ShaderType::FragmentShader, FS_SOURCE);
    if !ctx.shader_info(&fs).get_compile_status() {
        panic!(ctx.shader_info(&fs).get_info_log())
    }
    let program = ctx.new_program(&[vs, fs]);
    if !ctx.program_info(&program).get_link_status() {
        panic!(ctx.program_info(&program).get_info_log())
    }

    {
        let program_editor = ctx.edit_program(&program);
        let program_info = program_editor.program_info();
        let uniform_info = program_info.get_uniform_info();
        let scale_location = uniform_info.get_global_uniform("scale").map(|u| u.location).unwrap_or(-1);
        program_editor.uniform_f32(scale_location, 1, SimpleUniformTypeFloat::Uniform1f, &[1.5]);
        for uniform in uniform_info.globals.iter() {
            println!("{:?}", uniform);
        }
        for block in uniform_info.blocks.iter() {
            println!("InterfaceBlock {{ name: {}, index: {}, data_size: {} }}", block.name, block.index, block.data_size);
            for uniform in block.uniforms.iter() {
                println!("    {:?}", uniform);
            }
        }
        for attribute in program_info.get_attribute_info().attributes.iter() {
            println!("{:?}", attribute)
        }
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        let mut renderer = ctx.renderer();
        renderer.clear();
        renderer.use_vertex_array(&vao);
        renderer.use_program(&program);
        renderer.draw_elements_u16(PrimitiveMode::Triangles, 3, 0);

        window.swap_buffers();
        // break;
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
