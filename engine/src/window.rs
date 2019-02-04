// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod options;
mod surface;
mod update;

use crate::ecs;
use crate::math::Size;
use winit::Window as RawWindow;

pub use self::events::*;
pub use self::options::*;
pub use self::surface::{MaintainSurface, Surface};
pub use self::update::*;

pub struct Window {
  raw: RawWindow,
  size: Size<u32>,
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
    size: get_size_of(&raw),
    raw,
  };

  res.insert(window);
  res.insert(Events::default());

  UpdateWindow { events_loop }
}

fn get_size_of(window: &RawWindow) -> Size<u32> {
  let (width, height): (u32, u32) = window
    .get_inner_size()
    .expect("Could not get window size")
    .to_physical(window.get_hidpi_factor())
    .into();

  Size::new(width, height)
}
