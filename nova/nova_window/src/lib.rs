// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod options;
mod update;

pub use self::options::WindowOptions;
pub use self::update::UpdateWindow;
pub use winit::ElementState as ButtonState;
pub use winit::VirtualKeyCode as KeyCode;
pub use winit::{MouseButton, WindowEvent};

use nova_core::engine::{Engine, EnginePhase};
use nova_core::events::EventChannel;
use nova_core::math::Size;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use winit::EventsLoop;
use winit::Window as RawWindow;

pub type ReadWindow<'a> = ReadResource<'a, Window>;
pub type WriteWindow<'a> = WriteResource<'a, Window>;

pub struct Window {
  pub events: EventChannel<WindowEvent>,
  raw: RawWindow,
  size: Size<u32>,
}

impl Window {
  pub fn setup(engine: &mut Engine, options: WindowOptions) {
    if engine.resources.has_value::<Window>() {
      panic!("A window has already been set up.");
    }

    let events_loop = EventsLoop::new();

    let raw = winit::WindowBuilder::new()
      .with_title(options.title)
      .with_resizable(true)
      .with_dimensions(
        winit::dpi::PhysicalSize::new(options.size.width.into(), options.size.height.into())
          .to_logical(events_loop.get_primary_monitor().get_hidpi_factor()),
      )
      .build(&events_loop)
      .expect("Could not create window");

    let window = Window {
      events: EventChannel::new(),
      size: get_size_of(&raw),
      raw,
    };

    engine.resources.insert(window);

    engine.schedule_seq(EnginePhase::BeforeUpdate, UpdateWindow { events_loop });
  }

  pub fn raw(&self) -> &RawWindow {
    &self.raw
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }
}

pub fn borrow(res: &Resources) -> ReadWindow {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteWindow {
  resources::borrow_mut(res)
}

fn get_size_of(window: &RawWindow) -> Size<u32> {
  let (width, height): (u32, u32) = window
    .get_inner_size()
    .expect("Could not get window size")
    .to_physical(window.get_hidpi_factor())
    .into();

  Size::new(width, height)
}
