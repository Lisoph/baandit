mod gl;
#[macro_use] mod macros;
mod render;

use glfw::Context;
use font_kit as fk;

use std::sync::mpsc;

use render::{Renderer, UpdatedShaderSources};

struct Application {
    glfw: glfw::Glfw,
    window_event_rx: mpsc::Receiver<(f64, glfw::WindowEvent)>,
    window: glfw::Window,
    renderer: Renderer,
}

impl Application {
    fn run(&mut self) {
        'main: while !self.window.should_close() {
            self.renderer.frame();
            self.window.swap_buffers();

            // Poll our own events
            
            self.glfw.wait_events();
            for (_time, event) in self.window_event_rx.try_iter() {
                use glfw::WindowEvent::*;
                match event {
                    Key(glfw::Key::Escape, _, glfw::Action::Release, _) => break 'main,
                    _ => {},
                }
            }
        }
    }

    fn post_empty_event_handle<'a>(&'a self) -> PostEmptyEventHandle<'a> {
        PostEmptyEventHandle {
            window: &self.window,
        }
    }
}

struct PostEmptyEventHandle<'a> {
    window: &'a glfw::Window,
}

unsafe impl<'a> std::marker::Send for PostEmptyEventHandle<'a> {}
unsafe impl<'a> std::marker::Sync for PostEmptyEventHandle<'a> {}

impl<'a> PostEmptyEventHandle<'a> {
    fn post_empty_event(&self) {
        self.window.post_empty_event();
    }
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("glfw");

    let (mut window, window_event_rx) = glfw
        .create_window(720, 850, "Hello, world!", glfw::WindowMode::Windowed)
        .expect("Window");
    
    window.set_key_polling(true);
    window.set_char_polling(true);
    window.make_current();

    let gl = gl::Gl::load_with(|func| window.get_proc_address(func));
    let renderer = Renderer::new(gl);

    let mut app = Application {
        glfw,
        window_event_rx,
        window,
        renderer,
    };

    let peeh = app.post_empty_event_handle();

    include_str_reload!("shader/text_vert.glsl", move |code: Cow<'static, str>| {
        println!("{}", code);

        peeh.post_empty_event();

        // app.renderer.update_text_shader(UpdatedShaderSources {
        //     vertex: Some(code.as_ref()),
        //     ..Default::default()
        // });
        // app.window.post_empty_event();
    });

    app.run();
}