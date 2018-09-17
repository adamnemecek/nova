// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Component representing a sprite to be drawn.
#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Sprite {
  /// Atlas to source cells from.
  pub atlas: Arc<Atlas>,
  /// Cell in the atlas to render.
  pub cell: atlas::Cell,
  /// Scale of the sprite.
  pub scale: Vector2<f32>,
}

impl Sprite {
  pub fn new(atlas: Arc<Atlas>) -> Self {
    Sprite {
      atlas,
      cell: (0, 0),
      scale: Vector2::new(1.0, 1.0),
    }
  }
}
