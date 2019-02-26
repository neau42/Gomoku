use conrod::backend::glium::glium::glutin::Event;
use conrod::backend::glium::glium::glutin;

pub struct HomepageController {
}

impl HomepageController {
    pub fn new() -> HomepageController {
        HomepageController {
        }
    }

    pub fn event(&self, event: &Event) -> bool {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested | glutin::WindowEvent::KeyboardInput {
                    input: glutin::KeyboardInput {
                        virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => return true,
                _ => ()
            }
            _ => ()
        }
        false
    }
}