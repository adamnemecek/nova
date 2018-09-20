// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// System that changes the animation of an actor's object based on the actor's
/// current `State`.
pub struct Animator;

impl<'a> System<'a> for Animator {
  type SystemData = (ReadStorage<'a, Actor>, WriteStorage<'a, objects::Object>);

  fn run(&mut self, (actors, mut objects): Self::SystemData) {
    for (actor, object) in (&actors, &mut objects).join() {
      let index = actor.template.mode_animations[actor.mode.clone() as usize];

      if object.animation.index != index {
        object.animation.index = index;
        object.animation.elapsed = 0.0;
        object.animation.frame = 0;
      }
    }
  }
}
