// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

mod atlas;
mod sprite;

pub use self::atlas::*;
pub use self::sprite::*;

/// Sets up graphics for the given world.
pub fn setup<'a, 'b>(world: &mut World) {
  world.register::<sprite::Sprite>();
}
