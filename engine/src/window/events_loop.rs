// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Status, Window, WindowEvent};
use crate::ecs;
use crate::log;

pub struct EventsLoop {
  pub close_on_request: bool,
  events_loop: winit::EventsLoop,
  handle: Option<winit::Window>,
  log: log::Logger,
}

impl EventsLoop {
  pub fn new() -> Self {
    EventsLoop {
      close_on_request: true,
      events_loop: winit::EventsLoop::new(),
      handle: None,
      log: log::Logger::new("nova::window::EventsLoop"),
    }
  }
}

impl Default for EventsLoop {
  fn default() -> Self {
    EventsLoop::new()
  }
}

impl<'a> ecs::System<'a> for EventsLoop {
  type SystemData = ecs::WriteResource<'a, Window>;

  fn setup(&mut self, res: &mut ecs::Resources) {
    res.entry().or_insert_with(Window::default);
  }

  fn run(&mut self, mut window: Self::SystemData) {
    match window.status {
      Status::Opening => {
        let builder = winit::WindowBuilder::new().with_title(window.title.clone());
        let result = builder.build(&self.events_loop);

        match result {
          Ok(handle) => {
            self.handle = Some(handle);

            window.status = Status::Open;

            self.log.info("Opened.");
          }

          Err(err) => {
            window.status = Status::Closed;

            self.log.error("Could not open window.").with("err", err);

            return;
          }
        };
      }

      Status::Open | Status::Closing => {
        let handle = self.handle.as_ref().unwrap();

        handle.set_title(&window.title);

        let close_on_request = self.close_on_request;
        let was_closing = window.is_closing();
        let log = &self.log;

        self.events_loop.poll_events(|event| {
          if let winit::Event::WindowEvent { event, .. } = event {
            log.trace("Event.").with("event", &event);

            if let WindowEvent::CloseRequested = &event {
              if close_on_request {
                window.status = Status::Closing;
              }
            }

            window.events.single_write(event);
          }
        });

        if was_closing {
          self.handle = None;

          window.status = Status::Closed;

          self.log.info("Closed.");
        }
      }

      _ => {}
    }
  }
}
