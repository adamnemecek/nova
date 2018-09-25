// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::stage::actors::Actor;
use crate::stage::graphics::objects;

pub use super::objects::build_entity;

/// System that changes the animation of an actor's object based on the actor's
/// current `State`.
pub struct Animator;

impl<'a> System<'a> for Animator {
  type SystemData = (
    ReadStorage<'a, Actor>,
    WriteStorage<'a, objects::AnimationState>,
  );

  fn run(&mut self, (actors, mut anim_states): Self::SystemData) {
    for (actor, anim_state) in (&actors, &mut anim_states).join() {
      let index = actor.template.mode_animations[actor.mode.clone() as usize];

      if anim_state.animation != index {
        anim_state.animation = index;
        anim_state.frame_elapsed = 0.0;
        anim_state.frame = 0;
      }
    }
  }
}

/// Initializes visuals for actors in the given engine context.
pub fn init(ctx: &mut engine::Context) {
  engine::add_system(ctx, Animator, "stage::visuals::actors::Animator", &[]);
}
