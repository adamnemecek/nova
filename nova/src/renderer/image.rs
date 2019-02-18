// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::device::{Device, DeviceExt};
use super::Backend;
use crate::math::Size;

pub use gfx_hal::format::Format as ImageFormat;

pub type RawImage = <Backend as gfx_hal::Backend>::Image;
pub type RawImageView = <Backend as gfx_hal::Backend>::ImageView;

pub struct Image {
  pub(crate) raw: RawImage,
  pub(crate) raw_view: RawImageView,
  size: Size<u32>,
  in_swapchain: bool,
}

impl Image {
  pub(crate) fn new(device: &Device, size: Size<u32>) -> Self {
    const format: ImageFormat = ImageFormat::Rgba8Srgb;

    let raw = unsafe {
      device
        .create_image(
          gfx_hal::image::Kind::D2(size.width(), size.height(), 1, 1),
          1,
          format,
          gfx_hal::image::Tiling::Optimal,
          gfx_hal::image::Usage::TRANSFER_DST | gfx_hal::image::Usage::SAMPLED,
          gfx_hal::image::ViewCapabilities::KIND_2D_ARRAY,
        )
        .expect("Could not create image")
    };

    let raw_view = create_view(device, &raw, format);

    Image {
      raw,
      raw_view,
      size,
      in_swapchain: false,
    }
  }

  pub(crate) fn from_swapchain(
    device: &Device,
    raw: RawImage,
    format: ImageFormat,
    size: Size<u32>,
  ) -> Self {
    let raw_view = create_view(device, &raw, format);

    Image {
      raw,
      raw_view,
      size,
      in_swapchain: true,
    }
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }

  pub fn destroy(self, device: &Device) {
    unsafe {
      device.destroy_image_view(self.raw_view);

      if !self.in_swapchain {
        device.destroy_image(self.raw);
      }
    }
  }
}

fn create_view(device: &Device, raw_image: &RawImage, format: ImageFormat) -> RawImageView {
  unsafe {
    device
      .create_image_view(
        &raw_image,
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
  }
}
