// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod update;

pub use self::update::UpdateKeyboard;
pub use nova_window::KeyCode;

use nova_core::ecs;
use nova_core::engine::Engine;
use nova_core::events;
use std::mem;

const KEY_CODE_COUNT: usize = KeyCode::Cut as usize;

pub type ReadKeyboard<'a> = ecs::ReadResource<'a, Keyboard>;

type WriteKeyboard<'a> = ecs::WriteResource<'a, Keyboard>;

pub struct Keyboard {
  keys: [bool; KEY_CODE_COUNT],
  events: events::Channel<KeyboardEvent>,
}

impl Default for Keyboard {
  fn default() -> Self {
    Self {
      keys: [false; KEY_CODE_COUNT],
      events: events::Channel::new(),
    }
  }
}

impl Keyboard {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn get_key(&self, key: KeyCode) -> bool {
    self.keys[key as usize]
  }

  fn set_key(&mut self, key: KeyCode, value: bool) {
    let old_value = mem::replace(&mut self.keys[key as usize], value);

    if old_value != value {
      self
        .events
        .single_write(KeyboardEvent::KeyChanged { key, value });
    }
  }
}

#[derive(Debug)]
pub enum KeyboardEvent {
  KeyChanged { key: KeyCode, value: bool },
}

pub fn setup(engine: &mut Engine) {
  engine.resources.insert(Keyboard::default());

  update::setup(engine);
}
