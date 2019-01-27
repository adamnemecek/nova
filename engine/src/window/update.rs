// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Event, Window};
use crate::ecs;

pub struct Update;

impl<'a> ecs::System<'a> for Update {
  type SystemData = ecs::WriteResource<'a, Window>;

  fn run(&mut self, mut window: Self::SystemData) {
    while let Ok(event) = window.event_receiver.try_recv() {
      if let Event::Resized(size) = &event {
        let size: (u32, u32) = size.to_physical(window.handle.get_hidpi_factor()).into();

        window.options.size = size.into();
      }

      window.events.single_write(event);
    }
  }
}
