// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Backend, DeviceHandle, RawDeviceExt};
use crate::utils::Droppable;

pub use gfx_hal::format::Format as ImageFormat;

type RawImage = <Backend as gfx_hal::Backend>::Image;
type RawImageView = <Backend as gfx_hal::Backend>::ImageView;

pub struct Image {
  raw_view: Droppable<RawImageView>,
  _raw: RawImage,
  device: DeviceHandle,
}

impl Image {
  pub(crate) fn from_raw_image(device: &DeviceHandle, raw: RawImage, format: ImageFormat) -> Self {
    let raw_view = unsafe {
      device
        .raw()
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
      device: device.clone(),
      _raw: raw,
      raw_view: raw_view.into(),
    }
  }
}

impl Drop for Image {
  fn drop(&mut self) {
    if let Some(raw_view) = self.raw_view.take() {
      unsafe {
        self.device.raw().destroy_image_view(raw_view);
      }
    }
  }
}
