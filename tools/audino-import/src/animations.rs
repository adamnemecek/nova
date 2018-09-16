// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde_xml_rs;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn load(path: &Path) -> Result<AnimData, Box<dyn Error>> {
  let file = File::open(path)?;

  Ok(serde_xml_rs::deserialize(file)?)
}

#[derive(Debug, Deserialize)]
pub struct AnimData {
  #[serde(rename = "FrameWidth")]
  pub frame_width: usize,
  #[serde(rename = "FrameHeight")]
  pub frame_height: usize,
  #[serde(rename = "AnimGroupTable")]
  pub group_table: AnimGroupTable,
  #[serde(rename = "AnimSequenceTable")]
  pub sequence_table: AnimSequenceTable,
}

#[derive(Debug, Deserialize)]
pub struct AnimGroupTable {
  #[serde(rename = "AnimGroup")]
  pub groups: Vec<AnimGroup>,
}

#[derive(Debug, Deserialize)]
pub struct AnimGroup {
  #[serde(rename = "AnimSequenceIndex")]
  pub sequence_indices: Vec<usize>,
}

#[derive(Debug, Deserialize)]
pub struct AnimSequenceTable {
  #[serde(rename = "AnimSequence")]
  pub sequences: Vec<AnimSequence>,
}

#[derive(Debug, Deserialize)]
pub struct AnimSequence {
  #[serde(rename = "RushPoint")]
  pub rush_point: usize,
  #[serde(rename = "HitPoint")]
  pub hit_point: usize,
  #[serde(rename = "ReturnPoint")]
  pub return_point: usize,
  #[serde(rename = "AnimFrame")]
  pub frames: Vec<AnimFrame>,
}

#[derive(Debug, Deserialize)]
pub struct AnimFrame {
  #[serde(rename = "Duration")]
  pub duration: usize,
  #[serde(rename = "MetaFrameGroupIndex")]
  pub meta_frame_group_index: usize,
  #[serde(rename = "HFlip")]
  pub hflip: usize,
}

pub const DIRECTONS: [&'static str; 8] = ["s", "sw", "w", "nw", "n", "ne", "e", "se"];

pub enum Type {
  Static,
  Idle,
  Walk,
  Sleep,
  Hurt,
  Attack,
  Charge,
  Shoot,
  Strike,
  Chop,
  Scratch,
  Punch,
  Slap,
  Slice,
  MultiScratch,
  MultiStrike,
  Uppercut,
  Ricochet,
  Bite,
  Shake,
  Jab,
  Kick,
  Lick,
  Slam,
  Stomp,
  Appeal,
  Dance,
  Twirl,
  TailWhip,
  Sing,
  Sound,
  Rumble,
  FlapAround,
  Gas,
  Shock,
  Emit,
  Special,
  Withdraw,
  RearUp,
  Swell,
  Swing,
  Double,
  Rotate,
  Spin,
  Jump,
  HighJump,
}
