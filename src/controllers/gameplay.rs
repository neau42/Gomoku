use crate::controllers::homepage::HomepageController;
use crate::controllers::window::WindowController;
use crate::traits::view_controller::GameViewController;
use crate::traits::view_controller::PageType;
use crate::utils::event_loop::EventLoop as Events;
use crate::WidgetIds;

use conrod::backend::winit::convert_event;
use conrod::backend::glium::glium::glutin::*;
use conrod::backend::glium::glium::Display;
use conrod::backend::glium::glium::*;
use conrod::backend::glium::Renderer;
use glium::backend::glutin::DisplayCreationError;
use conrod::image::Map;
use conrod::*;
use conrod::glium::Surface;
use std::path::Path;

pub struct GameplayController {
    window: WindowController,
    page: Box<GameViewController>,
    ui: Ui,
    widget_ids: WidgetIds,
    events_loop: EventsLoop,
    width: u32,
    height: u32,
}

impl GameplayController {
    pub fn new(width: u32, height: u32, ui: Ui, widget_ids: WidgetIds) -> GameplayController {
        let window = WindowController::new();
        let page = HomepageController::new(&widget_ids);
        GameplayController {
            window,
            page,
            ui,
            widget_ids,
            events_loop: EventsLoop::new(),
            width,
            height,
        }
    }

    pub fn open_window(&self) -> Result<Display, DisplayCreationError> {
        let window_builder = WindowBuilder::new()
            .with_decorations(false)
            .with_dimensions((self.width, self.height).into());
        let context_builder = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        Display::new(window_builder, context_builder, &self.events_loop)
    }

    pub fn is_callback(&self, event: &Event) -> bool{
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

    pub fn render_loop(&mut self, display : Display, mut renderer: Renderer, image_map: Map<Texture2d>) {
        let mut events = Events::new();
        'render: loop {
            for event in events.next(&mut self.events_loop) {
                if let Some(event) = convert_event(event.clone(), &display) {
                    self.ui.handle_event(event);
                    events.needs_update();
                }
                if self.is_callback(&event) {
                   self.page = match self.page.get_type() {
                       PageType::Gameboard => HomepageController::new(&self.widget_ids),
                       _ => break 'render,
                   }
                }
            }
            if self.page.need_change_window() {
                self.page = match self.page.get_type() {
                    //Change byGameBoar::new
                    PageType::Homepage => HomepageController::new(&self.widget_ids),
                    _ => HomepageController::new(&self.widget_ids),
                }
            }
            
            let ui = &mut self.ui.set_widgets();
            self.window.show(ui, &self.widget_ids);
            self.page.show(ui, &self.widget_ids);

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

    pub fn run(&mut self) {
        let display = self.open_window().unwrap();
        let renderer = Renderer::new(&display).unwrap();
        let mut image_map = Map::<Texture2d>::new();
        
        self.window.load_background(&mut image_map, &display);
        self.ui.fonts.insert_from_file(Path::new("assets/fonts/FiraSans-Regular.ttf")).unwrap();
        self.render_loop(display, renderer, image_map);
    }
}
