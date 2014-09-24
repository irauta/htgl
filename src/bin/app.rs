
extern crate glfw;

extern crate mog;

use glfw::Context;

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    mog::load_with(|s| window.get_proc_address(s));

    let mut ctx = mog::Context::new();
    let vbo = ctx.new_vertex_buffer();
    let vertices = [-0.5f32,-0.5f32,0f32, 0.5f32,-0.5f32,0f32, 0f32,0.5f32,0f32];
    ctx.vertex_data(&vbo, &vertices);


    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
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