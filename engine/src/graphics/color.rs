// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use derive_more::*;

#[derive(Debug, Clone, Copy, From)]
pub struct Color4 {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color4 {
  pub const TRANSPARENT: Self = Color4::new(0.0, 0.0, 0.0, 0.0);
  pub const WHITE: Self = Color4::new(1.0, 1.0, 1.0, 1.0);

  /// Creates a new color with the given component values.
  pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Color4 { r, g, b, a }
  }
}

// Implement `From` to convert from arrays of color components.
impl From<[f32; 4]> for Color4 {
  fn from(values: [f32; 4]) -> Self {
    Color4::new(values[0], values[1], values[2], values[3])
  }
}
