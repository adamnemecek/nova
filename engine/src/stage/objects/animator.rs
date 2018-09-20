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
      let state = &mut object.animation;
      let template = object.template.as_ref();

      // Determine the sequence index based on object direction.
      let sequence_index = if object.template.cardinal_dirs_only {
        stage::CompassDirection::nearest_cardinal(object.facing.remove_row(2))
      } else {
        stage::CompassDirection::nearest(object.facing.remove_row(2))
      } as usize;

      let sequence = get_animation_sequence(template, state.index, sequence_index)
        .or_else(|| get_animation_sequence(template, 0, sequence_index));

      if let Some(sequence) = sequence {
        state.elapsed += time::seconds(clock.delta_time);
        state.sequence = sequence_index;

        // Determine total duration of the sequence for wrapping.
        let mut duration = 0.0;

        for frame in sequence {
          duration += frame.length;
        }

        // Determine current frame based on wrapped elapsed time.
        let mut elapsed = state.elapsed * 60.0;

        if duration > 0.0 {
          elapsed %= duration;
        }

        // Find the current frame.
        for (i, frame) in sequence.iter().enumerate() {
          elapsed -= frame.length;

          if elapsed <= 0.0 {
            state.frame = i;
            break;
          }
        }
      }
    }
  }
}

fn get_animation_sequence(
  template: &Template,
  animation: usize,
  sequence: usize,
) -> Option<&Vec<AnimationFrame>> {
  template
    .animations
    .get(animation)
    .and_then(|anim| anim.sequences.get(sequence))
    .and_then(|seq| seq.as_ref())
}
