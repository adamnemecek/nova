// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod update;

pub use self::update::UpdateKeyboard;
pub use nova_window::KeyCode;

use nova_core::engine::Engine;
use nova_core::events::EventChannel;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::mem;

const KEY_CODE_COUNT: usize = KeyCode::Cut as usize;

pub type ReadKeyboard<'a> = ReadResource<'a, Keyboard>;
pub type WriteKeyboard<'a> = WriteResource<'a, Keyboard>;

pub struct Keyboard {
  pub events: EventChannel<KeyboardEvent>,
  keys: [bool; KEY_CODE_COUNT],
}

impl Default for Keyboard {
  fn default() -> Self {
    Self {
      keys: [false; KEY_CODE_COUNT],
      events: EventChannel::new(),
    }
  }
}

impl Keyboard {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn borrow(res: &Resources) -> ReadKeyboard {
    resources::borrow(res)
  }

  pub fn borrow_mut(res: &Resources) -> WriteKeyboard {
    resources::borrow_mut(res)
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

pub fn set_up(engine: &mut Engine) {
  engine.resources.insert(Keyboard::default());

  update::set_up(engine);
}
