// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend::Backend;
use crate::gpu::{Gpu, GpuDeviceExt};
use crate::images::{Image, ImageFormat, ImageId, Images};
use crate::surfaces::Surface;
use nova_core::math::{clamp, Size};
use std::cmp;

type BackendSwapchain = <Backend as gfx_hal::Backend>::Swapchain;

pub struct Swapchain {
  swapchain: BackendSwapchain,
  image_ids: Vec<ImageId>,
}

impl Swapchain {
  pub fn new(
    gpu: &Gpu,
    surface: &mut Surface,
    images: &mut Images,
    format: ImageFormat,
    size: Size<u32>,
  ) -> Swapchain {
    let capabilities = surface.capabilities(gpu);

    let extent = gfx_hal::window::Extent2D {
      width: clamp(
        size.width,
        capabilities.extents.start.width..capabilities.extents.end.width,
      ),
      height: clamp(
        size.height,
        capabilities.extents.start.height..capabilities.extents.end.height,
      ),
    };

    let image_count = match capabilities.image_count.end {
      0 => 2, // Any number of images is allowed. Only need two.
      x => cmp::min(x, 2),
    };

    let config = gfx_hal::SwapchainConfig {
      present_mode: gfx_hal::window::PresentMode::Fifo,
      format,
      extent,
      image_count,
      image_layers: 1,
      image_usage: gfx_hal::image::Usage::COLOR_ATTACHMENT,
      composite_alpha: gfx_hal::window::CompositeAlpha::Opaque,
    };

    let (swapchain, backbuffers) = unsafe {
      gpu
        .device
        .create_swapchain(surface.as_hal_mut(), config, None)
        .expect("Could not create swapchain")
    };

    let actual_size = Size::new(extent.width, extent.height);
    let mut image_ids = Vec::with_capacity(image_count as usize);

    match backbuffers {
      gfx_hal::Backbuffer::Images(imgs) => {
        for img in imgs {
          let image = Image::new_view(&gpu, img, format, actual_size);
          let id = images.insert(image);

          image_ids.push(id);
        }
      }

      // I think this only happens with OpenGL, which isn't supported.
      _ => panic!("Device created framebuffer objects."),
    };

    Swapchain {
      swapchain,
      image_ids,
    }
  }

  pub fn destroy(self, gpu: &Gpu, images: &mut Images) {
    for image_id in self.image_ids {
      images.destroy_view(gpu, image_id);
    }

    unsafe {
      gpu.device.destroy_swapchain(self.swapchain);
    }
  }
}
