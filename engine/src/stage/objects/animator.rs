// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// System that animates an object by changing its sprite based on animation
/// sequences in the object's template.
#[derive(Default)]
pub struct Animator;

impl<'a> System<'a> for Animator {
  type SystemData = (Read<'a, time::Clock>, WriteStorage<'a, Object>);

  fn run(&mut self, (clock, mut objects): Self::SystemData) {
    for object in (&mut objects).join() {
      // Progress the animation time.
      object.animation.elapsed += time::seconds(clock.delta_time);

      // Get the object's template's animations.
      let animations = &object.template.animations;

      // Skip objects with no animations.
      if object.animation.index >= animations.len() {
        continue;
      }

      // Get the current animation.
      let animation = &animations[object.animation.index];

      // Determine the direction index of the current sequence in the animation.
      let direction = if object.template.cardinal_dirs_only {
        stage::CompassDirection::nearest_cardinal(object.facing.remove_row(2))
      } else {
        stage::CompassDirection::nearest(object.facing.remove_row(2))
      } as usize;

      object.animation.sequence = direction;

      // Get the current sequence or go to the next object.
      if let Some(ref sequence) = animation.sequences[direction] {
        if sequence.len() == 0 {
          continue;
        }

        // Determine total duration of the sequence for wrapping.
        let mut duration = 0.0;

        for frame in sequence {
          duration += frame.length;
        }

        // Determine current frame based on wrapped elapsed time.
        let mut elapsed = object.animation.elapsed * 60.0;

        if duration > 0.0 {
          elapsed %= duration;
        }

        object.animation.frame = 0;

        for (i, frame) in sequence.iter().enumerate() {
          elapsed -= frame.length;

          if elapsed <= 0.0 {
            object.animation.frame = i;
            break;
          }
        }
      }
    }
  }
}
