// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Image;
use nova_math::Rect;

#[derive(Debug, Clone, PartialEq)]
pub struct ImageSlice {
  image: Image,
  rect: Rect<f32>,
}

impl ImageSlice {
  pub fn new(image: Image, rect: Rect<f32>) -> Self {
    Self { image, rect }
  }

  pub fn image(&self) -> &Image {
    &self.image
  }

  pub fn rect(&self) -> &Rect<f32> {
    &self.rect
  }
}

impl From<Image> for ImageSlice {
  fn from(image: Image) -> Self {
    Self {
      image,
      rect: Rect {
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
      },
    }
  }
}

impl From<&Image> for ImageSlice {
  fn from(image: &Image) -> Self {
    image.clone().into()
  }
}