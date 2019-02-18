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
  size: Size<u32>,
  pub(crate) raw_view: RawImageView,
  #[allow(dead_code)]
  pub(crate) raw: RawImage,
}

impl Image {
  pub(crate) fn from_raw(
    device: &Device,
    raw: RawImage,
    format: ImageFormat,
    size: Size<u32>,
  ) -> Self {
    let raw_view = unsafe {
      device
        .create_image_view(
          &raw,
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

    Image {
      raw,
      raw_view,
      size,
    }
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }

  pub fn destroy(self, device: &Device) {
    unsafe {
      device.destroy_image_view(self.raw_view);
    }
  }
}
