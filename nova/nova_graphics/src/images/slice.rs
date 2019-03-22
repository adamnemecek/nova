// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::images::ImageData;
use nova_core::math::Rect;

#[derive(Debug, Clone, PartialEq)]
pub struct ImageSlice {
  pub data: ImageData,
  pub rect: Rect<f32>,
}

impl From<&ImageData> for ImageSlice {
  fn from(data: &ImageData) -> Self {
    Self {
      data: data.clone(),
      rect: Rect {
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
      },
    }
  }
}
