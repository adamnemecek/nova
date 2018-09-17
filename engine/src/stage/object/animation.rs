// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use super::*;

#[derive(Default, Debug)]
pub struct Animation {
  pub name: String,
  pub sequences: [Option<Sequence>; stage::direction::COMPASS_DIRECTION_COUNT],
}

impl From<Data> for Animation {
  fn from(mut data: Data) -> Animation {
    let mut animation = Animation::default();

    animation.name = data.name;

    for (i, direction) in DIRECTION_NAMES.iter().enumerate() {
      animation.sequences[i] = data
        .sequences
        .remove(*direction)
        .map(|frames| Sequence { frames });
    }

    animation
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  pub name: String,
  #[serde(flatten)]
  pub sequences: HashMap<String, Vec<Frame>>,
}

#[derive(Debug)]
pub struct Sequence {
  pub frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
  pub length: f64,
  pub cell: graphics::atlas::Cell,
  #[serde(default)]
  pub hflip: bool,
}

pub const DIRECTION_NAMES: [&'static str; 8] = [
  "south",
  "southwest",
  "west",
  "northwest",
  "north",
  "northeast",
  "east",
  "southeast",
];

#[derive(Default, Component)]
#[storage(BTreeStorage)]
pub struct Animated {
  pub animation: usize,
  pub elapsed: f64,
}

#[derive(Default)]
pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
  type SystemData = (
    Read<'a, core::Clock>,
    ReadStorage<'a, Object>,
    WriteStorage<'a, Animated>,
    WriteStorage<'a, graphics::Sprite>,
  );

  fn run(&mut self, (clock, objects, mut animated, mut sprites): Self::SystemData) {
    for (object, animated, sprite) in (&objects, &mut animated, &mut sprites).join() {
      animated.elapsed += clock.delta_time;

      let animations = &object.template.animations;

      if animated.animation >= animations.len() {
        continue;
      }

      let direction = if object.template.cardinal_dirs_only {
        stage::CompassDirection::nearest_cardinal(object.facing.remove_row(2))
      } else {
        stage::CompassDirection::nearest(object.facing.remove_row(2))
      };

      if let Some(ref sequence) = animations[animated.animation].sequences[direction as usize] {
        if sequence.frames.len() == 0 {
          continue;
        }

        // Determine total duration of the sequence for wrapping.
        let mut duration = 0.0;

        for frame in &sequence.frames {
          duration += frame.length;
        }

        // Determine current frame based on wrapped elapsed time.
        let mut current = &sequence.frames[0];
        let mut elapsed = animated.elapsed * 60.0;

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

        sprite.scale = if current.hflip {
          Vector2::new(-1.0, 1.0)
        } else {
          Vector2::new(1.0, 1.0)
        };
      }
    }
  }
}
