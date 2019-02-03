// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod options;
mod surface;

use crate::ecs;
use crate::engine;
use crate::math::Size;
use winit::Window as RawWindow;

pub use self::events::*;
pub use self::options::*;
pub use self::surface::{MaintainSurface, Surface};

pub struct Window {
  raw: RawWindow,
  size: Size<u32>,
}

pub struct UpdateWindow {
  events_loop: EventsLoop,
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
            window.size = get_size(&window.raw);
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

pub fn setup(res: &mut ecs::Resources, options: Options) -> UpdateWindow {
  if res.has_value::<Window>() {
    panic!("A window has already been set up.");
  }

  let events_loop = EventsLoop::new();

  let raw = winit::WindowBuilder::new()
    .with_title(options.title)
    .with_dimensions(
      winit::dpi::PhysicalSize::new(options.size.width().into(), options.size.height().into())
        .to_logical(events_loop.get_primary_monitor().get_hidpi_factor()),
    )
    .build(&events_loop)
    .expect("Could not create window");

  let window = Window {
    size: get_size(&raw),
    raw,
  };

  res.insert(window);
  res.insert(Events::default());

  UpdateWindow { events_loop }
}

fn get_size(window: &RawWindow) -> Size<u32> {
  let (width, height): (u32, u32) = window
    .get_inner_size()
    .expect("Could not get window size")
    .to_physical(window.get_hidpi_factor())
    .into();

  Size::new(width, height)
}
