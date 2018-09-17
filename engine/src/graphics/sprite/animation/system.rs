// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::Animation;

/// System that animates `Sprite` components.
#[derive(Default)]
pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
  type SystemData = (
    Read<'a, core::Clock>,
    WriteStorage<'a, Animation>,
    WriteStorage<'a, graphics::Sprite>,
  );

  fn run(&mut self, (clock, mut animations, mut sprites): Self::SystemData) {
    // For all sprites that are animated…
    for (animation, sprite) in (&mut animations, &mut sprites).join() {
      match animation.sequence {
        Some(ref sequence) if sequence.frames.len() > 0 => {
          // Elapse the animation by delta time.
          animation.elapsed += clock.delta_time;

          // Determine total duration of the sequence for wrapping.
          let mut duration = 0.0;

          for frame in &sequence.frames {
            duration += frame.length;
          }

          // Determine current frame based on wrapped elapsed time.
          let mut current = &sequence.frames[0];
          let mut elapsed = animation.elapsed * 60.0;

          if duration > 0.0 {
            elapsed %= duration;
          }

          for frame in &sequence.frames {
            elapsed -= frame.length;

            if elapsed <= 0.0 {
              current = frame;
              break;
            }
          }

          // Update the sprite with the current frame data.
          sprite.cell = current.cell;
          sprite.hflip = current.hflip;
        }

        _ => {
          // Set the elapsed time to zero if nothing to animate.
          animation.elapsed = 0.0;
        }
      }
    }
  }
}
