// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// System that animates an object by changing its sprite based on animation
/// sequences in the object's template.
#[derive(Default)]
pub struct Animator;

impl<'a> System<'a> for Animator {
  type SystemData = (
    Read<'a, time::Clock>,
    WriteStorage<'a, Object>,
    WriteStorage<'a, graphics::Sprite>,
  );

  fn run(&mut self, (clock, mut objects, mut sprites): Self::SystemData) {
    for (object, sprite) in (&mut objects, &mut sprites).join() {
      // Progress the animation time.
      object.animation.elapsed += time::seconds(clock.delta_time);

      // Get the object's template's animations or go to the next object.
      let animations = &object.template.animations;

      if object.animation.index >= animations.len() {
        continue;
      }

      // Get the compass direction the object is facing.
      let direction = if object.template.cardinal_dirs_only {
        stage::CompassDirection::nearest_cardinal(object.facing.remove_row(2))
      } else {
        stage::CompassDirection::nearest(object.facing.remove_row(2))
      };

      // Get the current sequence or go to the next object.
      if let Some(ref sequence) = animations[object.animation.index].sequences[direction as usize] {
        if sequence.len() == 0 {
          continue;
        }

        // Determine total duration of the sequence for wrapping.
        let mut duration = 0.0;

        for frame in sequence {
          duration += frame.length;
        }

        // Determine current frame based on wrapped elapsed time.
        let mut current = &sequence[0];
        let mut elapsed = object.animation.elapsed * 60.0;

        if duration > 0.0 {
          elapsed %= duration;
        }

        for frame in sequence {
          elapsed -= frame.length;

          if elapsed <= 0.0 {
            current = frame;
            break;
          }
        }

        // Update the sprite with the current frame data.
        sprite.cell = current.cell;

        sprite.scale = if current.hflip {
          Vector2::new(-1.0, 1.0)
        } else {
          Vector2::new(1.0, 1.0)
        };

        sprite.offset = current.offset;
      }
    }
  }
}
