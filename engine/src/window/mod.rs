// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod options;

use crate::ecs;
use std::sync::Arc;

pub use self::options::Options;
pub use winit::WindowEvent as Event;
pub use winit::{EventsLoop, Window};

pub fn create(options: Options) -> (Arc<Window>, EventsLoop) {
  let events_loop = EventsLoop::new();

  let monitor = events_loop.get_primary_monitor();

  let window = winit::WindowBuilder::new()
    .with_title(options.title)
    .with_dimensions(
      winit::dpi::PhysicalSize::new(options.size.width().into(), options.size.height().into())
        .to_logical(monitor.get_hidpi_factor()),
    )
    .build(&events_loop)
    .expect("Could not create window")
    .into();

  (window, events_loop)
}

pub type EventChannel = crate::events::Channel<Event>;

#[derive(Default)]
pub struct Events {
  channel: EventChannel,
}

pub struct PollEvents {
  pub events_loop: EventsLoop,
}

impl<'a> ecs::System<'a> for PollEvents {
  type SystemData = ecs::WriteResource<'a, Events>;

  fn setup(&mut self, res: &mut ecs::Resources) {
    res.entry().or_insert_with(Events::default);
  }

  fn run(&mut self, mut events: Self::SystemData) {
    self.events_loop.poll_events(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        events.channel.single_write(event);
      }
    });
  }
}
