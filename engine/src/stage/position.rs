// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;
use std::ops::{Deref, DerefMut};

/// Component that stores the position of an entity on the stage.
#[derive(Component, Clone, Copy)]
#[storage(BTreeStorage)]
pub struct Position(pub Point3<f32>);

impl Default for Position {
  fn default() -> Self {
    Position(Point3::new(0.0, 0.0, 0.0))
  }
}

// Implement deref ops so it's easy to use position with nalgebra types.
impl Deref for Position {
  type Target = Point3<f32>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Position {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
