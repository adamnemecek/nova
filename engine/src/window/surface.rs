// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::RawWindow;
use crate::graphics;

pub use gfx_hal::Surface as RawSurfaceExt;
pub use gfx_hal::SurfaceCapabilities;

type RawSurface = <graphics::Backend as gfx_hal::Backend>::Surface;

pub struct Surface {
  raw: RawSurface,
  device: graphics::Device,
}

impl Surface {
  pub(super) fn new(window: &RawWindow, device: &graphics::Device) -> Self {
    let surface = device.backend().create_surface(window);

    Surface {
      raw: surface,
      device: device.clone(),
    }
  }

  pub(super) fn raw(&self) -> &RawSurface {
    &self.raw
  }

  pub(super) fn raw_mut(&mut self) -> &mut RawSurface {
    &mut self.raw
  }

  pub(super) fn device(&self) -> &graphics::Device {
    &self.device
  }

  pub(super) fn capabilities(&self) -> SurfaceCapabilities {
    let (capabilities, _, _, _) = self.raw.compatibility(self.device.raw_physical());

    capabilities
  }
}
