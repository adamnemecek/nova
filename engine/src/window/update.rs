// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Status, Window};
use crate::ecs;
use crate::log;

pub fn update() -> Update {
  Update {
    events_loop: winit::EventsLoop::new(),
    log: log::Logger::new("nova::window::update"),
  }
}

pub struct Update {
  events_loop: winit::EventsLoop,
  log: log::Logger,
}

impl<'a> ecs::System<'a> for Update {
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
            window.handle = Some(handle);
            window.status = Status::Open;

            self.log.info("Opened window.");
          }

          Err(err) => {
            window.status = Status::Closed;

            self.log.error("Could not open window.").with("err", err);

            return;
          }
        };
      }

      Status::Open | Status::Closing => {
        let handle = window.handle.as_ref().unwrap();

        handle.set_title(&window.title);

        self.events_loop.poll_events(|event| {
          if let winit::Event::WindowEvent { event, .. } = event {
            window.events.single_write(event);
          }
        });

        if let Status::Closing = window.status {
          window.handle = None;
          window.status = Status::Closed;

          self.log.info("Closed window.");
        }
      }

      _ => {}
    }
  }
}
