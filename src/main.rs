mod event_loop;
mod controllers;
mod views;

use ::image::open;
use conrod::*;
use conrod::glium::Surface;
use std::path::Path;
use conrod::backend::glium::glium::glutin::*;
use conrod::backend::glium::glium::Display;
use conrod::backend::winit::convert_event;
use views::homepage::*;
use controllers::homepage::*;
use gomoku::gui::color::fill_color;
use conrod::color;
const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1024;

widget_ids! {
    pub struct WidgetIndex {
        background,
        title,
        window_canvas,
        window_canvas_y_scrollbar,
        window_canvas_x_scrollbar,
        homepage_canvas,
        button_player_vs_player,
        button_player_vs_ia,
        toggle_button_weight_boxes,
        dropdown_button_deph,
        text,
    }
}

fn load_background(display: &glium::Display) -> glium::texture::Texture2d {
    let rgba_image = open(&Path::new("assets/images/wood.jpg")).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
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
    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    ui.fonts.insert_from_file(Path::new("assets/fonts/FiraSans-Regular.ttf")).unwrap();
    
    let backgroud = image_map.insert(load_background(&display));

    let mut events = event_loop::EventLoop::new();
    let homepage_controller = HomepageController::new();
    let homepage_view = HomepageView::new(&mut ui);
    let widget_index = WidgetIndex::new(ui.widget_id_generator());

    'render: loop {
         for event in events.next(&mut events_loop) {
            if let Some(event) = convert_event(event.clone(), &display) {
                ui.handle_event(event);
                events.needs_update();
            }
            if homepage_controller.event(&event) {
                break 'render;
            }
        }

        let ui = &mut ui.set_widgets();

        widget::Image::new(backgroud).wh_of(ui.window).middle_of(ui.window).set(widget_index.background, ui);
        widget::Canvas::new()
            .border(1.0)
            .pad(50.0)
            .color(color::TRANSPARENT)
            .scroll_kids()
            .set(widget_index.window_canvas, ui);
        widget::Scrollbar::x_axis(widget_index.window_canvas).auto_hide(true).set(widget_index.window_canvas_y_scrollbar, ui);
        widget::Scrollbar::y_axis(widget_index.window_canvas).auto_hide(true).set(widget_index.window_canvas_x_scrollbar, ui);
        widget::Text::new("Gomoku")
            .mid_top_of(widget_index.window_canvas)
            .font_size(50)
            .color(color::BLACK)
            .set(widget_index.title, ui);
        homepage_view.display(ui, &widget_index);
        
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
