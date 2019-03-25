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
use nova_graphics::surfaces::{Surface, Swapchain};

pub type ReadWindow<'a> = ReadResource<'a, Window>;
pub type WriteWindow<'a> = WriteResource<'a, Window>;

pub struct Window {
  pub events: EventChannel<WindowEvent>,
  window: Option<winit::Window>,
  size: Size<u32>,
  surface: Option<Surface>,
  swapchain: Option<Swapchain>,
}

impl Window {
  pub fn size(&self) -> Size<u32> {
    self.size
  }

  fn refresh_size(&mut self) {
    if let Some(window) = &mut self.window {
      let (width, height): (u32, u32) = window
        .get_inner_size()
        .expect("Could not get window size")
        .to_physical(window.get_hidpi_factor())
        .into();

      self.size = Size::new(width, height);
    }
  }
}

pub fn set_up(engine: &mut Engine, options: WindowOptions) {
  if engine.resources.has_value::<Window>() {
    return;
  }

  nova_graphics::setup(&mut engine.resources).expect("Could not set up graphics");

  let events_loop = winit::EventsLoop::new();

  let window = winit::WindowBuilder::new()
    .with_title(options.title)
    .with_resizable(true)
    .with_dimensions(
      winit::dpi::PhysicalSize::new(options.size.width.into(), options.size.height.into())
        .to_logical(events_loop.get_primary_monitor().get_hidpi_factor()),
    )
    .build(&events_loop)
    .expect("Could not create window");

  let surface = Surface::new(&engine.resources, &window).expect("Could not create window surface");

  let window = Window {
    events: EventChannel::new(),
    window: Some(window),
    size: Size::default(),
    surface: Some(surface),
    swapchain: None,
  };

  engine.resources.insert(window);

  engine.schedule_seq(EnginePhase::BeforeUpdate, UpdateWindow { events_loop });
}

pub fn borrow(res: &Resources) -> ReadWindow {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteWindow {
  resources::borrow_mut(res)
}
