mod event_loop;
// #[macro_use] extern crate conrod_core;
#[macro_use]
extern crate conrod;
use crate::event_loop::*;
use piston_window::*;
use conrod::*;

// use conrod_core::{widget, Colorable, Positionable, Widget};
// use conrod;
use opengl_graphics::{GlyphCache, Filter};
use std::path::Path;
use conrod::backend::glium::glium::glutin;
use conrod::glium::Surface;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    let mut events_loop = conrod::backend::glium::glium::glutin::EventsLoop::new();
    
    let window_builder = conrod::backend::glium::glium::glutin::WindowBuilder::new()
        .with_decorations(false)
        .with_dimensions((WIDTH, HEIGHT).into());
    
    let context_builder = conrod::backend::glium::glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    
    let display = conrod::backend::glium::glium::Display::new(window_builder, context_builder, &events_loop).unwrap();
    
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    // let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings).expect("Could not load font");
    ui.fonts.insert_from_file(Path::new("assets/FiraSans-Regular.ttf")).unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();


    // The image map describing each of our widget->image mappings (in our case, none).
     let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();


    // Generate the widget identifiers.
        widget_ids! { struct Ids {
        text
    } }
    let ids = Ids::new(ui.widget_id_generator());


    let mut event_loop = event_loop::EventLoop::new();
    'render: loop {
         for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
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
