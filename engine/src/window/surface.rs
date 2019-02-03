// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Window;
use crate::ecs;
use crate::graphics;
use crate::math::Size;

type RawSurface = <graphics::Backend as gfx_hal::Backend>::Surface;

pub struct Surface {
  _raw: RawSurface,
  _device: graphics::DeviceHandle,
  size: Size<u32>,
}

impl Surface {
  fn new(window: &Window, device: &graphics::DeviceHandle) -> Self {
    let surface = device.backend().create_surface(&window.raw);

    Surface {
      _raw: surface,
      _device: device.clone(),
      size: window.size,
    }
  }

  fn _acquire_backbuffer(&mut self) {
    for _ in 0..5 {
      self._ensure_created();

      self.destroy();
    }

    panic!("Swapchain was repeatedly out of date.");
  }

  fn _ensure_created(&mut self) {}

  fn destroy(&mut self) {}
}

pub struct MaintainSurface;

impl<'a> ecs::System<'a> for MaintainSurface {
  type SystemData = (
    ecs::ReadResource<'a, Window>,
    ecs::WriteResource<'a, Surface>,
  );

  fn setup(&mut self, res: &mut ecs::Resources) {
    res.insert({
      let device = res.fetch();
      let window = res.fetch();

      Surface::new(&window, &device)
    });
  }

  fn run(&mut self, (window, mut surface): Self::SystemData) {
    if surface.size != window.size {
      surface.destroy();
    }
  }
}
