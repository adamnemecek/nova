// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use winit::ElementState as ButtonState;
pub use winit::VirtualKeyCode as KeyCode;
pub use winit::{EventsLoop, MouseButton, WindowEvent};

use nova_core::events;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};

pub type ReadWindowEvents<'a> = ReadResource<'a, WindowEvents>;
pub type WriteWindowEvents<'a> = WriteResource<'a, WindowEvents>;

#[derive(Default)]
pub struct WindowEvents {
  channel: events::Channel<WindowEvent>,
}

impl WindowEvents {
  pub fn channel(&self) -> &events::Channel<WindowEvent> {
    &self.channel
  }

  pub fn channel_mut(&mut self) -> &mut events::Channel<WindowEvent> {
    &mut self.channel
  }
}

pub fn borrow(res: &Resources) -> ReadWindowEvents {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteWindowEvents {
  resources::borrow_mut(res)
}
