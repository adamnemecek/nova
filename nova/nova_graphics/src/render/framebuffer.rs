// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::Gpu;
use crate::images::Image;
use crate::render::RenderPass;
use crate::Backend;
use gfx_hal::Device as _;
use nova_core::collections::SmallVec;
use nova_core::math::Size;
use std::cmp;

type HalFramebuffer = <Backend as gfx_hal::Backend>::Framebuffer;

pub struct Framebuffer {
  framebuffer: HalFramebuffer,
  size: Size<u32>,
}

impl Framebuffer {
  pub fn new<'a>(
    gpu: &Gpu,
    render_pass: &RenderPass,
    images: impl IntoIterator<Item = &'a Image>,
  ) -> Self {
    let mut attachments = SmallVec::<[_; 4]>::new();

    let mut extent = gfx_hal::image::Extent {
      depth: 1,
      ..Default::default()
    };

    for image in images {
      let size = image.size();

      extent.width = cmp::max(extent.width, size.width);
      extent.height = cmp::max(extent.height, size.height);

      attachments.push(image.as_hal_view());
    }

    let framebuffer = unsafe {
      gpu
        .device
        .create_framebuffer(render_pass.as_hal(), attachments, extent)
        .expect("Could not create framebuffer")
    };

    Self {
      framebuffer,
      size: Size::new(extent.width, extent.height),
    }
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }

  pub fn destroy(self, gpu: &Gpu) {
    unsafe { gpu.device.destroy_framebuffer(self.framebuffer) };
  }

  pub(crate) fn as_hal(&self) -> &HalFramebuffer {
    &self.framebuffer
  }
}
