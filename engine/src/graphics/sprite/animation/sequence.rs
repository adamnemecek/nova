// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use graphics::sprite::*;

#[derive(Serialize, Deserialize)]
pub struct Sequence {
  pub name: String,
  pub frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize)]
pub struct Frame {
  /// Length of this frame in 60ths of a second.
  pub length: f64,
  /// Which cell in the atlas to draw.
  pub cell: atlas::Cell,
  /// Whether to flip the sprite horizontally.
  #[serde(default)]
  pub hflip: bool,
}
