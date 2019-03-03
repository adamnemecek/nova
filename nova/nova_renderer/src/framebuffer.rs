// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::device::{Device, DeviceExt};
use super::presenter::Backbuffer;
use super::{Backend, RenderPass};
use nova_math::Size;

pub type RawFramebuffer = <Backend as gfx_hal::Backend>::Framebuffer;

pub struct Framebuffer {
  pub(crate) raw: RawFramebuffer,
  size: Size<u32>,
}

impl Framebuffer {
  pub fn new(device: &Device, render_pass: &RenderPass, backbuffer: &Backbuffer) -> Framebuffer {
    let raw = unsafe {
      device
        .create_framebuffer(
          &render_pass,
          Some(&backbuffer.view),
          gfx_hal::image::Extent {
            width: backbuffer.size.width,
            height: backbuffer.size.height,
            depth: 1,
          },
        )
        .expect("Could not create framebuffer")
    };

    Framebuffer {
      raw,
      size: backbuffer.size,
    }
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }

  pub fn destroy(self, device: &Device) {
    unsafe {
      device.destroy_framebuffer(self.raw);
    }
  }
}
