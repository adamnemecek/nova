// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::collections::HashMap;

pub const KEY_CODE_COUNT: usize = KeyCode::Cut as usize + 1;

/// Asset and resource that stores bindings from `KeyCode` to `Button`.
///
/// Each key can be assigned to zero or more buttons.
#[derive(Debug)]
pub struct Mapping {
  /// List of key bindngs in order of `KeyCode`.
  keys: Vec<Vec<Button>>,
}

impl Mapping {
  /// Creates a new, empty `Mapping`.
  pub fn new() -> Mapping {
    let mut mapping = Mapping {
      keys: Vec::with_capacity(KEY_CODE_COUNT),
    };

    for _ in 0..KEY_CODE_COUNT {
      mapping.keys.push(Vec::with_capacity(2));
    }

    mapping
  }

  /// Gets the buttons bound to the given key code.
  pub fn get(&self, key_code: KeyCode) -> &Vec<Button> {
    &self.keys[key_code as usize]
  }

  /// Get a mutable reference to the buttons bound to the given key code.
  pub fn get_mut(&mut self, key_code: KeyCode) -> &mut Vec<Button> {
    &mut self.keys[key_code as usize]
  }
}

// Set up the default mapping assuming standard QWERTY controls.
impl Default for Mapping {
  fn default() -> Self {
    let mut mapping = Mapping::new();

    mapping.get_mut(KeyCode::W).push(Button::Up);
    mapping.get_mut(KeyCode::A).push(Button::Left);
    mapping.get_mut(KeyCode::S).push(Button::Down);
    mapping.get_mut(KeyCode::D).push(Button::Right);

    mapping.get_mut(KeyCode::Up).push(Button::Up);
    mapping.get_mut(KeyCode::Left).push(Button::Left);
    mapping.get_mut(KeyCode::Down).push(Button::Down);
    mapping.get_mut(KeyCode::Right).push(Button::Right);

    mapping
  }
}

// Implement `Asset` for `Mapping` by delegating to `MappingData`.
impl assets::Asset for Mapping {
  fn load(fs: &assets::OverlayFs, path: &assets::Path) -> Result<Self, assets::Error> {
    fs.load::<MappingData>(path).map(Mapping::from)
  }
}

// Support creting `Mapping` from `MappingData`.
impl From<MappingData> for Mapping {
  fn from(data: MappingData) -> Mapping {
    let mut mapping = Mapping::new();

    for (key, buttons) in data.keys {
      if let Some(key) = parse_key_code(&key) {
        mapping.keys[key as usize] = buttons;
      }
    }

    mapping
  }
}

/// Serializable data for a `Mapping` asset.
#[derive(Serialize, Deserialize)]
pub struct MappingData {
  #[serde(flatten)]
  pub keys: HashMap<String, Vec<Button>>,
}

/// Returns the key code referred to by the given string or `None` if it does
/// not match a key code.
fn parse_key_code(value: &str) -> Option<KeyCode> {
  Some(match value {
    "1" => KeyCode::Key1,
    "2" => KeyCode::Key2,
    "3" => KeyCode::Key3,
    "4" => KeyCode::Key4,
    "5" => KeyCode::Key5,
    "6" => KeyCode::Key6,
    "7" => KeyCode::Key7,
    "8" => KeyCode::Key8,
    "9" => KeyCode::Key9,

    "A" => KeyCode::A,
    "B" => KeyCode::B,
    "C" => KeyCode::C,
    "D" => KeyCode::D,
    "E" => KeyCode::E,
    "F" => KeyCode::F,
    "G" => KeyCode::G,
    "H" => KeyCode::H,
    "I" => KeyCode::I,
    "J" => KeyCode::J,
    "K" => KeyCode::K,
    "L" => KeyCode::L,
    "M" => KeyCode::M,
    "N" => KeyCode::N,
    "O" => KeyCode::O,
    "P" => KeyCode::P,
    "Q" => KeyCode::Q,
    "R" => KeyCode::R,
    "S" => KeyCode::S,
    "T" => KeyCode::T,
    "U" => KeyCode::U,
    "V" => KeyCode::V,
    "W" => KeyCode::W,
    "X" => KeyCode::X,
    "Y" => KeyCode::Y,
    "Z" => KeyCode::Z,

    "Up" => KeyCode::Up,
    "Left" => KeyCode::Left,
    "Down" => KeyCode::Down,
    "Right" => KeyCode::Right,

    _ => return None,
  })
}
