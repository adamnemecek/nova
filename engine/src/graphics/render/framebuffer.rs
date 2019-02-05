// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Pass;
use crate::graphics::{Backend, Device, Image, RawDeviceExt};
use crate::math::Size;
use crate::utils::Droppable;

type RawFramebuffer = <Backend as gfx_hal::Backend>::Framebuffer;

pub struct Framebuffer {
  raw: Droppable<RawFramebuffer>,
  device: Device,
  size: Size<u32>,
}

impl Framebuffer {
  pub fn new(pass: &Pass, image: &Image) -> Self {
    let device = pass.device().clone();

    let raw = unsafe {
      device
        .raw()
        .create_framebuffer(pass.raw(), Some(image.raw_view()), image.size().into())
        .expect("Could not create framebuffer")
    };

    Framebuffer {
      device,
      raw: raw.into(),
      size: image.size(),
    }
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }

  pub(crate) fn raw(&self) -> &RawFramebuffer {
    &self.raw
  }
}

impl Drop for Framebuffer {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      unsafe {
        self.device.raw().destroy_framebuffer(raw);
      }
    }
  }
}
