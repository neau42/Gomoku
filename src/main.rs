mod event_loop;
mod controllers;
mod views;

use conrod::*;
use conrod::backend::glium::glium::glutin;
use conrod::glium::Surface;
use std::path::Path;
use conrod::backend::glium::glium::glutin::*;
use conrod::backend::glium::glium::Display;
use conrod::backend::winit::convert_event;
// use crate::views::homepage_view::*;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1024;


widget_ids! {
    struct Ids {
        text
    }
}

//  Renderer = A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
//  for drawing to the glium `Surface`.

// The image map describing each of our widget->image mappings (in our case, none).

fn main() {
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new()
        .with_decorations(false)
        .with_dimensions((WIDTH, HEIGHT).into());
    let context_builder = ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = Display::new(window_builder, context_builder, &events_loop).unwrap();
    let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    ui.fonts.insert_from_file(Path::new("assets/FiraSans-Regular.ttf")).unwrap();

    let mut events = event_loop::EventLoop::new();

    let ids = Ids::new(ui.widget_id_generator());
    // let homepage_controller = HomepageController::new();
    // let homepage_view = HomepageView::new(&ui);

    'render: loop {
         for event in events.next(&mut events_loop) {
            if let Some(event) = convert_event(event.clone(), &display) {
                ui.handle_event(event);
                events.needs_update();
            }
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested | glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'render,
                    _ => ()
                }
                _ => ()
            }
            // homepage_controller.event(&event);
        }

        // Set the widgets.
        let ui = &mut ui.set_widgets();

        // "Hello World!" in the middle of the screen.
        widget::Text::new("Hello World!")
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(ids.text, ui);

        // Draw the `Ui` if it has changed.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
