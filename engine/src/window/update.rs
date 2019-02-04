// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{get_size_of, Event, Events, EventsLoop, Window};
use crate::ecs;
use crate::engine;

pub struct UpdateWindow {
  pub(super) events_loop: EventsLoop,
}

impl<'a> ecs::System<'a> for UpdateWindow {
  type SystemData = (
    ecs::WriteResource<'a, Window>,
    ecs::WriteResource<'a, Events>,
    ecs::WriteResource<'a, engine::Exit>,
  );

  fn run(&mut self, (mut window, mut events, mut exit): Self::SystemData) {
    self.events_loop.poll_events(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        match event {
          Event::Resized(_) => {
            window.size = get_size_of(&window.raw);
          }

          Event::CloseRequested => {
            exit.requested = true;
          }

          _ => {}
        };

        events.channel_mut().single_write(event);
      }
    });
  }
}
