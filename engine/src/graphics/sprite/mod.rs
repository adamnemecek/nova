// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use prelude::*;

pub mod animation;
pub mod atlas;

pub use self::animation::{Animation, AnimationSystem};
pub use self::atlas::Atlas;

/// Component representing a sprite to be drawn.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Sprite {
  /// Atlas to source cells from.
  pub atlas: Arc<Atlas>,
  /// Cell in the atlas to render.
  pub cell: atlas::Cell,
  /// Whether to flip the sprite horizontally.
  pub hflip: bool,
}

impl Sprite {
  pub fn new(atlas: Arc<Atlas>) -> Self {
    Sprite {
      atlas,
      cell: (0, 0),
      hflip: false,
    }
  }
}
