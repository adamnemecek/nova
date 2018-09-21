// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::collections::HashMap;

pub const KEY_CODE_COUNT: usize = KeyCode::Cut as usize + 1;

/// Asset and resource that stores bindings from `Button` to `KeyCode`.
///
/// Each button can be assigned zero or more key codes.
#[derive(Debug)]
pub struct Mapping {
  mappings: Vec<Vec<KeyCode>>,
  reverse: Vec<Vec<Button>>,
}

impl Mapping {
  /// Creates a new, empty `Mapping`.
  pub fn new() -> Mapping {
    Mapping {
      mappings: std::iter::repeat_with(Vec::new)
        .take(BUTTON_COUNT)
        .collect(),
      reverse: std::iter::repeat_with(Vec::new)
        .take(KEY_CODE_COUNT)
        .collect(),
    }
  }

  /// Adds a mapping from `button` to `key`.
  pub fn add(&mut self, button: Button, key: KeyCode) {
    self.mappings[button as usize].push(key);
    self.reverse[key as usize].push(button);
  }

  /// Gets the buttons bound to the given key code.
  pub fn get_buttons_for(&self, key_code: KeyCode) -> &Vec<Button> {
    &self.reverse[key_code as usize]
  }
}

// Set up the default mapping assuming standard QWERTY controls.
impl Default for Mapping {
  fn default() -> Self {
    let mut mapping = Mapping::new();

    mapping.add(Button::Up, KeyCode::W);
    mapping.add(Button::Up, KeyCode::Up);
    mapping.add(Button::Left, KeyCode::A);
    mapping.add(Button::Left, KeyCode::Left);
    mapping.add(Button::Down, KeyCode::S);
    mapping.add(Button::Down, KeyCode::Down);
    mapping.add(Button::Right, KeyCode::D);
    mapping.add(Button::Right, KeyCode::Right);

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

    for (button, keys) in data.mappings {
      for key in keys {
        if let Some(key) = parse_key_code(&key) {
          mapping.add(button, key);
        }
      }
    }

    mapping
  }
}

/// Serializable data for a `Mapping` asset.
#[derive(Serialize, Deserialize)]
pub struct MappingData {
  #[serde(flatten)]
  pub mappings: HashMap<Button, Vec<String>>,
}

/// Sets the input mapping. Discards the current mapping.
pub fn set_mapping(world: &mut World, mapping: Mapping) {
  world.add_resource(mapping);
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
