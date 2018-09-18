// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::collections::HashMap;

/// Animation for an object.
#[derive(Default, Debug)]
pub struct Animation {
  /// Name of the animation.
  pub name: String,
  /// Array of optional sequences, one sequence per compass direction.
  pub sequences: [Option<Sequence>; stage::direction::COMPASS_DIRECTION_COUNT],
}

// Create animations from loaded data.
impl From<Data> for Animation {
  fn from(mut data: Data) -> Animation {
    // Create a new `Animation` with the name in the data.
    let mut animation = Animation::default();

    animation.name = data.name;

    // Get a sequence from the data for each compass direction by name.
    for (i, direction) in direction::COMPASS_DIRECTION_NAMES.iter().enumerate() {
      animation.sequences[i] = data
        .sequences
        .remove(*direction)
        .map(|frames| Sequence { frames });
    }

    animation
  }
}

/// Data for an `Animation`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  /// Name of the animation.
  pub name: String,
  /// Map of sequences where each key is the name of a compass direction.
  #[serde(flatten)]
  pub sequences: HashMap<String, Vec<Frame>>,
}

/// Sequence of frames in an `Animation`.
#[derive(Debug)]
pub struct Sequence {
  pub frames: Vec<Frame>,
}

/// Single frame in a `Sequence`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
  #[serde(default)]
  /// Length of this frame in 60ths of a second.
  pub length: f64,
  /// Cell in the atlas to use as the object's sprite during this frame.
  pub cell: graphics::atlas::Cell,
  /// Whether the object's sprite is flipped during this frame.
  #[serde(default)]
  pub hflip: bool,
}
