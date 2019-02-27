// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{get_size_of, Event, Events, EventsLoop, Window};
use nova_core::ecs;

pub struct UpdateWindow {
  pub(crate) events_loop: EventsLoop,
}

impl<'a> ecs::System<'a> for UpdateWindow {
  type SystemData = (
    ecs::WriteResource<'a, Window>,
    ecs::WriteResource<'a, Events>,
  );

  fn run(&mut self, (mut window, mut events): Self::SystemData) {
    self.events_loop.poll_events(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        if let Event::Resized(_) = event {
          window.size = get_size_of(&window.raw);
        };

        events.channel_mut().single_write(event);
      }
    });
  }
}
