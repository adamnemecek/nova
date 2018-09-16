// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Serialize, Deserialize)]
pub struct Animation {
  pub name: String,
  pub frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize)]
pub struct Frame {
  /// Which cell in the atlas to draw.
  pub cell: usize,
  /// Length of this frame in 60ths of a second.
  pub length: f64,
  /// Whether to flip the sprite horizontally.
  #[serde(default)]
  pub hflip: bool,
}
