// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Window;
use crate::graphics;

pub use gfx_hal::Surface as RawSurfaceExt;
pub use gfx_hal::SurfaceCapabilities;

type RawSurface = <graphics::Backend as gfx_hal::Backend>::Surface;

pub struct Surface {
  raw: RawSurface,
  device: graphics::Device,
}

impl Surface {
  pub(crate) fn new(window: &Window, device: &graphics::Device) -> Self {
    let surface = device.backend().create_surface(&window.raw);

    Surface {
      raw: surface,
      device: device.clone(),
    }
  }

  pub(crate) fn raw(&self) -> &RawSurface {
    &self.raw
  }

  pub(crate) fn raw_mut(&mut self) -> &mut RawSurface {
    &mut self.raw
  }

  pub(crate) fn capabilities(&self) -> SurfaceCapabilities {
    let (capabilities, _, _, _) = self.raw.compatibility(self.device.raw_physical());

    capabilities
  }
}
