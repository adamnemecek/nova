// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Component that describes an entity's sprite representation.
#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Sprite {
  /// Source `Atlas` for the sprite graphics.
  pub atlas: Arc<Atlas>,
  /// Cell of the atlas to get the sprite from.
  pub cell: atlas::AtlasCell,
  /// Scale of the sprite in x- and y-directions.
  pub scale: Vector2<f32>,
  /// Offset from the atlas cell origin when drawing the sprite.
  pub offset: Vector2<f32>,
}

impl Sprite {
  pub fn new(atlas: Arc<Atlas>) -> Self {
    Sprite {
      atlas,
      cell: (0, 0),
      scale: Vector2::new(1.0, 1.0),
      offset: Vector2::zeros(),
    }
  }
}
