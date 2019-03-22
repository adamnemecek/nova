// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::events::{EventsLoop, WindowEvent, WriteWindowEvents};
use crate::{get_size_of, WriteWindow};
use nova_core::systems::System;

#[derive(Debug)]
pub struct UpdateWindow {
  pub(crate) events_loop: EventsLoop,
}

impl<'a> System<'a> for UpdateWindow {
  type Data = (WriteWindow<'a>, WriteWindowEvents<'a>);

  fn run(&mut self, (mut window, mut events): Self::Data) {
    self.events_loop.poll_events(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        if let WindowEvent::Resized(_) = event {
          window.size = get_size_of(&window.raw);
        };

        events.channel_mut().single_write(event);
      }
    });
  }
}
