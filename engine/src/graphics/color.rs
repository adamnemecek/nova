// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug, Clone, Copy)]
pub struct Color(pub [f32; 4]);

impl Color {
  pub const TRANSPARENT: Self = Color([0.0, 0.0, 0.0, 0.0]);
  pub const WHITE: Self = Color([1.0, 1.0, 1.0, 1.0]);

  pub fn a(&self) -> f32 {
    self.0[0]
  }
}
