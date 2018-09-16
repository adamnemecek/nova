// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Data for an animation in an atlas.
#[derive(Serialize, Deserialize)]
pub struct Animation {
  /// Name of the animation.
  pub name: String,
  /// List of frames in the animation.
  pub frames: Vec<Frame>,
}

/// Data for one frame of an atlas animation.
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
