// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::backend;
use crate::graphics::device::DeviceHandle;
use crate::math::Size;
use crate::window::{self, Window};

/// A rendering surface created from a [`Window`].
pub struct Surface {
  raw: backend::Surface,
  window: window::Handle,
  device: DeviceHandle,
}

impl Surface {
  /// Creates a new surface from a window with the given backend instance.
  pub fn new(device: &DeviceHandle, window: &Window) -> Surface {
    let surface = device.backend().create_surface(window.handle().as_ref());

    Surface {
      raw: surface,
      window: window.handle().clone(),
      device: device.clone(),
    }
  }

  /// Gets a reference to the device the surface was created with.
  pub fn device(&self) -> &DeviceHandle {
    &self.device
  }

  /// Determines the current size of the surface in pixels.
  pub fn get_size(&self) -> Size<u32> {
    self.window.get_size()
  }
}

// Implement `AsRef` and `AsMut` to expose the raw backend surface.
impl AsRef<backend::Surface> for Surface {
  fn as_ref(&self) -> &backend::Surface {
    &self.raw
  }
}

impl AsMut<backend::Surface> for Surface {
  fn as_mut(&mut self) -> &mut backend::Surface {
    &mut self.raw
  }
}
