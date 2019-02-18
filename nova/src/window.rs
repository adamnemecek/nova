// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod options;
mod update;

use crate::engine::{self, Engine};
use crate::math::Size;
use winit::Window as RawWindow;

pub use self::events::*;
pub use self::options::*;
pub use self::update::*;

pub struct Window {
  pub(crate) raw: RawWindow,
  size: Size<u32>,
}

impl Window {
  pub fn size(&self) -> Size<u32> {
    self.size
  }
}

pub fn setup(engine: &mut Engine, options: Options) {
  if engine.resources().has_value::<Window>() {
    panic!("A window has already been set up.");
  }

  let events_loop = EventsLoop::new();

  let raw = winit::WindowBuilder::new()
    .with_title(options.title)
    .with_resizable(false)
    .with_dimensions(
      winit::dpi::PhysicalSize::new(options.size.width().into(), options.size.height().into())
        .to_logical(events_loop.get_primary_monitor().get_hidpi_factor()),
    )
    .build(&events_loop)
    .expect("Could not create window");

  let window = Window {
    size: get_size_of(&raw),
    raw,
  };

  engine.resources_mut().insert(window);
  engine.resources_mut().insert(Events::default());

  engine.on_event(engine::Event::TickStarted, UpdateWindow { events_loop });
}

fn get_size_of(window: &RawWindow) -> Size<u32> {
  let (width, height): (u32, u32) = window
    .get_inner_size()
    .expect("Could not get window size")
    .to_physical(window.get_hidpi_factor())
    .into();

  Size::new(width, height)
}
