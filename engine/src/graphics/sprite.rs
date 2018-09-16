// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use prelude::*;

use super::Atlas;

/// Component representing a sprite to be drawn.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Sprite {
  /// Atlas to source cells from.
  pub atlas: Arc<Atlas>,
  /// Cell in the atlas to render.
  pub cell: usize,
  /// Whether to flip the sprite horizontally.
  pub hflip: bool,
}

/// Component indicating that entity has an animated sprite.
#[derive(Default, Component)]
#[storage(BTreeStorage)]
pub struct Animated {
  /// Index of the animation in the atlas to play.
  pub animation: Option<usize>,
  /// Elapsed time in the animation.
  pub elapsed: f64,
}

/// System that animates sprite components.
#[derive(Default)]
pub struct Animator;

impl<'a> System<'a> for Animator {
  type SystemData = (
    Read<'a, core::Clock>,
    WriteStorage<'a, Animated>,
    WriteStorage<'a, Sprite>,
  );

  fn run(&mut self, (clock, mut animated, mut sprites): Self::SystemData) {
    // For all sprites that are animated…
    for (state, sprite) in (&mut animated, &mut sprites).join() {
      match state.animation {
        Some(animation) if animation < sprite.atlas.data.animations.len() => {
          let animation = &sprite.atlas.data.animations[animation];

          if animation.frames.len() == 0 {
            continue;
          }

          // Elapse the animation by delta time.
          state.elapsed += clock.delta_time;

          // Determine total duration of animation for wrapping.
          let mut duration = 0.0;

          for frame in &animation.frames {
            duration += frame.length;
          }

          // Determine current frame based on wrapped elapsed time.
          let mut current = &animation.frames[0];
          let mut elapsed = state.elapsed * 60.0;

          if duration > 0.0 {
            elapsed %= duration;
          }

          for frame in &animation.frames {
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

        _ => {}
      }
    }
  }
}
