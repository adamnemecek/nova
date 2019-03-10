// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::math::Size;
use std::f32;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Constraints {
  pub fn narrow_by(&self, constraints: Constraints) -> Self {
    Constraints {
      min: constraints.constrain(self.min),
      max: constraints.constrain(self.max),
    }
  }

  pub fn constrain(&self, size: Size<f32>) -> Size<f32> {
    Size {
      width: size.width.max(self.min.width).min(self.max.width),
      height: size.height.max(self.min.height).min(self.max.height),
    }
  }

  pub fn largest_finite_size(&self) -> Size<f32> {
    Size {
      width: if self.max.width.is_finite() {
        self.max.width
      } else if self.min.width.is_finite() {
        self.min.width
      } else {
        0.0
      },

      height: if self.max.height.is_finite() {
        self.max.height
      } else if self.min.height.is_finite() {
        self.min.height
      } else {
        0.0
      },
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

impl Mul<f32> for Constraints {
  type Output = Self;

  fn mul(self, by: f32) -> Self {
    Constraints {
      min: self.min * by,
      max: self.max * by,
    }
  }
}
