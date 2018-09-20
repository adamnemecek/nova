use serde_xml_rs;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub const GROUP_NAMES: [&'static str; 3] = ["static", "idle", "walk"];

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

/// Data from an `animations.xml` file about the animations for a monster.
#[derive(Debug, Deserialize)]
pub struct AnimData {
  /// Width of a cell in the sprite sheet.
  #[serde(rename = "FrameWidth")]
  pub frame_width: usize,
  /// Height of a cell in the sprite sheet.
  #[serde(rename = "FrameHeight")]
  pub frame_height: usize,
  /// Table of animation groups defining individual animations.
  #[serde(rename = "AnimGroupTable")]
  pub group_table: AnimGroupTable,
  /// Table of sequences defining individual animation sequences referenced in
  /// groups.
  #[serde(rename = "AnimSequenceTable")]
  pub sequence_table: AnimSequenceTable,
}

impl AnimData {
  /// Loads an `animations.xml` file as `AnimData`.
  pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
    let file = File::open(path)?;

    Ok(serde_xml_rs::deserialize(file)?)
  }
}

/// Table of animation groups defining individual animations.
#[derive(Debug, Deserialize)]
pub struct AnimGroupTable {
  /// List of animation groups in the table.
  #[serde(rename = "AnimGroup")]
  pub groups: Vec<AnimGroup>,
}

/// Animation group, which defines an individual animation.
#[derive(Debug, Deserialize)]
pub struct AnimGroup {
  /// List of sequence indices, with one element in the list for each direction,
  /// that define this animation.
  #[serde(rename = "AnimSequenceIndex")]
  pub sequence_indices: Vec<usize>,
}

/// Table of animation sequences defining individual animation sequences used in
/// groups.
#[derive(Debug, Deserialize)]
pub struct AnimSequenceTable {
  /// List of animation sequences in the table.
  #[serde(rename = "AnimSequence")]
  pub sequences: Vec<AnimSequence>,
}

/// An animation sequence, which defines a sequence of frames for use in an
/// animation group.
#[derive(Debug, Deserialize)]
pub struct AnimSequence {
  /// For attack animations, the frame to rush forward.
  #[serde(rename = "RushPoint")]
  pub rush_point: usize,
  /// For attack animations, the frame the rush hits.
  #[serde(rename = "HitPoint")]
  pub hit_point: usize,
  /// For attack animations, the frame the rush completes.
  #[serde(rename = "ReturnPoint")]
  pub return_point: usize,
  /// List of frames in the sequence.
  #[serde(rename = "AnimFrame")]
  pub frames: Vec<AnimFrame>,
}

// An animation frame.
#[derive(Debug, Deserialize)]
pub struct AnimFrame {
  /// Duration of this frame in 60ths of a second.
  #[serde(rename = "Duration")]
  pub duration: usize,
  /// Index of the sprite in the sprite sheet to use during this frame.
  #[serde(rename = "MetaFrameGroupIndex")]
  pub meta_frame_group_index: usize,
  /// Whether the sprite is horizontally flipped during this frame.
  #[serde(rename = "HFlip")]
  pub hflip: i32,
  /// Sprite offset on this frame.
  #[serde(rename = "Sprite")]
  pub sprite: Offset,
}

#[derive(Debug, Deserialize)]
pub struct Offset {
  #[serde(rename = "XOffset")]
  pub x_offset: i32,
  #[serde(rename = "YOffset")]
  pub y_offset: i32,
}

/// Animation types in order of index in audino data files.
#[allow(dead_code)]
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
