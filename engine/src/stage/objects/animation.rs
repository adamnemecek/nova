// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::stage::{COMPASS_DIRECTION_COUNT, COMPASS_DIRECTION_NAMES};
use std::collections::HashMap;

/// Animation for an object.
#[derive(Default, Debug)]
pub struct Animation {
  /// Name of the animation.
  pub name: String,
  /// Array of optional sequences, one sequence per compass direction.
  pub sequences: [Option<Vec<AnimationFrame>>; COMPASS_DIRECTION_COUNT],
}

// Create animations from loaded data.
impl From<AnimationData> for Animation {
  fn from(mut data: AnimationData) -> Animation {
    // Create a new `Animation` with the name in the data.
    let mut animation = Animation::default();

    animation.name = data.name;

    // Get a sequence from the data for each compass direction by name.
    for (i, direction) in COMPASS_DIRECTION_NAMES.iter().enumerate() {
      animation.sequences[i] = data
        .sequences
        .remove(*direction)
        .map(|frames| frames.into_iter().map(AnimationFrame::from).collect());
    }

    animation
  }
}

#[derive(Debug)]
pub struct AnimationFrame {
  /// Length of this frame in 60ths of a second.
  pub length: f64,
  /// Cell in the atlas to use as the object's sprite during this frame.
  pub cell: Vector2<u16>,
  /// Visual offset to apply to the sprite during this frame.
  pub offset: Vector2<f32>,
  /// Whether the object's sprite is flipped during this frame.
  pub hflip: bool,
}

// Create animation frames from loaded data.
impl From<AnimationFrameData> for AnimationFrame {
  fn from(data: AnimationFrameData) -> AnimationFrame {
    AnimationFrame {
      length: data.length,
      cell: Vector2::new(data.cell.0, data.cell.1),
      offset: Vector2::new(data.offset.0, data.offset.1),
      hflip: data.hflip,
    }
  }
}

/// Data for an `Animation`.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationData {
  /// Name of the animation.
  pub name: String,
  /// Map of sequences where each key is the name of a compass direction.
  #[serde(flatten)]
  pub sequences: HashMap<String, Vec<AnimationFrameData>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationFrameData {
  /// Cell in the atlas to use as the object's sprite during this frame.
  pub cell: (u16, u16),
  /// Length of this frame in 60ths of a second.
  #[serde(default)]
  pub length: f64,
  /// Visual offset to apply to the sprite during this frame.
  #[serde(default)]
  pub offset: (f32, f32),
  /// Whether the object's sprite is flipped during this frame.
  #[serde(default)]
  pub hflip: bool,
}
