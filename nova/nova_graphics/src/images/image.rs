// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::{Gpu, GpuDeviceExt};
use crate::images::ImageFormat;
use crate::Backend;
use nova_core::math::Size;

type HalImage = <Backend as gfx_hal::Backend>::Image;
type HalImageView = <Backend as gfx_hal::Backend>::ImageView;

#[derive(Debug)]
pub struct Image {
  image: HalImage,
  view: HalImageView,
  size: Size<u32>,
  is_owned: bool,
}

impl Image {
  pub(crate) fn new_view(gpu: &Gpu, image: HalImage, format: ImageFormat, size: Size<u32>) -> Self {
    let view = unsafe {
      gpu
        .device
        .create_image_view(
          &image,
          gfx_hal::image::ViewKind::D2,
          format,
          gfx_hal::format::Swizzle::NO,
          gfx_hal::image::SubresourceRange {
            aspects: gfx_hal::format::Aspects::COLOR,
            levels: 0..1,
            layers: 0..1,
          },
        )
        .expect("Could not create image view")
    };

    Self {
      image,
      view,
      size,
      is_owned: false,
    }
  }

  pub fn destroy(self, gpu: &Gpu) {
    unsafe {
      gpu.device.destroy_image_view(self.view);

      if self.is_owned {
        gpu.device.destroy_image(self.image);
      }
    }
  }

  pub(crate) fn destroy_view(self, gpu: &Gpu) {
    debug_assert!(!self.is_owned, "destroy_view called on a regular image");

    unsafe {
      gpu.device.destroy_image_view(self.view);
    }
  }
}
