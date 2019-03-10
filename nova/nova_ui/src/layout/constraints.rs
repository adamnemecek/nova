// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::math::Size;
use std::f32;

#[derive(Debug, Clone, Copy)]
pub struct Constraints {
  pub min: Size<f32>,
  pub max: Size<f32>,
}

impl Default for Constraints {
  fn default() -> Self {
    Self {
      min: Size::default(),
      max: Size::new(f32::INFINITY, f32::INFINITY),
    }
  }
}

impl From<Size<f32>> for Constraints {
  fn from(size: Size<f32>) -> Self {
    Constraints {
      min: size,
      max: size,
    }
  }
}
