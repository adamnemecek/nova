// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use winit::EventsLoop;
pub use winit::WindowEvent as Event;

use nova_core::events;

pub type EventChannel = events::Channel<Event>;
pub type EventReader = events::ReaderId<Event>;

#[derive(Default)]
pub struct Events {
  channel: EventChannel,
}

impl Events {
  pub fn channel(&self) -> &EventChannel {
    &self.channel
  }

  pub fn channel_mut(&mut self) -> &mut EventChannel {
    &mut self.channel
  }
}
