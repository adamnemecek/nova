// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod options;

pub use self::events::*;
pub use self::options::*;
pub use winit::Window;

pub fn create(options: Options) -> (Window, EventsLoop) {
  let events_loop = EventsLoop::new();

  let monitor = events_loop.get_primary_monitor();

  let window = winit::WindowBuilder::new()
    .with_title(options.title)
    .with_dimensions(
      winit::dpi::PhysicalSize::new(options.size.width().into(), options.size.height().into())
        .to_logical(monitor.get_hidpi_factor()),
    )
    .build(&events_loop)
    .expect("Could not create window");

  (window, events_loop)
}
