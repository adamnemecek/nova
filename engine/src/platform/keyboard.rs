// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use ggez::event::KeyCode;
use std::collections::HashMap;

#[derive(Default)]
pub struct Keyboard(HashMap<KeyCode, f64>);

impl Keyboard {
  pub fn set_pressed(&mut self, key: KeyCode, time: f64) {
    self.0.insert(key, time);
  }

  pub fn set_released(&mut self, key: KeyCode) {
    self.0.remove(&key);
  }

  pub fn is_pressed(&self, key: KeyCode) -> bool {
    self.0.contains_key(&key)
  }

  pub fn get_pressed_time(&self, key: KeyCode) -> Option<f64> {
    self.0.get(&key).map(|k| *k)
  }
}
