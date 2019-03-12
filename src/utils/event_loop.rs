use conrod::backend::glium::glium::glutin;
use std::{thread, time::Duration};

pub struct EventLoop {
    ui_needs_update: bool,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            ui_needs_update: true,
        }
    }

    pub fn next(&mut self, events_loop: &mut glutin::EventsLoop) -> Vec<glutin::Event> {
        let mut events = vec![];
        let proxy = events_loop.create_proxy();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(16));
            proxy.wakeup().ok(); // wakeup can fail only if the event loop went away
        });
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glutin::ControlFlow::Break
            });
        }
        self.ui_needs_update = false;
        events
    }

    pub fn needs_update(&mut self) { self.ui_needs_update = true; }
}
