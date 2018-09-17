// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub mod sprite;

pub use self::sprite::Sprite;

/// Sets up graphics components, resources, and systems.
pub fn setup<'a, 'b>(core: &mut Core, dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<sprite::Sprite>();
  core.world.register::<sprite::Animation>();

  dispatch.add(
    sprite::AnimationSystem::default(),
    "graphics::sprite::AnimationSystem",
    &[],
  );
}
