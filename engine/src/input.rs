use std::collections::HashMap;

pub use ggez::input::keyboard::KeyCode;

use time;

#[derive(Default)]
pub struct Keyboard {
  pressed: HashMap<KeyCode, time::Tick>,
}

impl Keyboard {
  pub fn set_pressed(&mut self, key: KeyCode, when: time::Tick) {
    self.pressed.insert(key, when);
  }

  pub fn set_released(&mut self, key: KeyCode) {
    self.pressed.remove(&key);
  }

  pub fn is_pressed(&self, key: KeyCode) -> bool {
    self.pressed.contains_key(&key)
  }
}
