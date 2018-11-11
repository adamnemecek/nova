use super::{AnimationState, Sprite};
use crate::prelude::*;
use crate::stage::objects::{AnimationFrame, Object, Template};
use crate::stage::CompassDirection;
use nova::time::Clock;

/// System that animates an object by changing its sprite based on animation
/// sequences in the object's template.
#[derive(Default)]
pub struct Animator;

impl<'a> System<'a> for Animator {
  type SystemData = (
    ReadResource<'a, Clock>,
    ReadStorage<'a, Object>,
    WriteStorage<'a, AnimationState>,
    WriteStorage<'a, Sprite>,
  );

  fn run(&mut self, (clock, objects, mut states, mut sprites): Self::SystemData) {
    for (object, state, sprite) in (&objects, &mut states, &mut sprites).join() {
      // Determine the sequence direction index.
      let direction = if object.template.cardinal_dirs_only {
        CompassDirection::nearest_cardinal(object.facing.remove_row(2))
      } else {
        CompassDirection::nearest(object.facing.remove_row(2))
      } as usize;

      let sequence = get_animation_sequence(&object.template, state.animation, direction)
        .or_else(|| get_animation_sequence(&object.template, 0, direction));

      if let Some(sequence) = sequence {
        state.sequence = direction;
        state.frame_elapsed += clock.delta_time * 60.0;

        // Ignore empty sequences.
        if sequence.len() == 0 {
          continue;
        }

        // Progress through frames based on elapsed time.
        loop {
          while state.frame >= sequence.len() {
            state.frame -= sequence.len();
          }

          let frame = &sequence[state.frame];

          if frame.length == 0.0 || state.frame_elapsed < frame.length {
            sprite.cell = frame.cell;
            sprite.offset = frame.offset;
            sprite.hflip = frame.hflip;

            break;
          }

          state.frame_elapsed -= frame.length;
          state.frame += 1;
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
