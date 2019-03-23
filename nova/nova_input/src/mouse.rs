// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod update;

pub use self::update::UpdateMouse;

use nova_core::engine::Engine;
use nova_core::events::EventChannel;
use nova_core::math::Point2;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use serde::{Deserialize, Serialize};
use std::iter;
use std::mem;

pub type ReadMouse<'a> = ReadResource<'a, Mouse>;
pub type WriteMouse<'a> = WriteResource<'a, Mouse>;

#[derive(Default)]
pub struct Mouse {
  pub events: EventChannel<MouseEvent>,
  buttons: Vec<bool>,
  position: Option<Point2<f32>>,
}

impl Mouse {
  pub fn borrow(res: &Resources) -> ReadMouse {
    resources::borrow(res)
  }

  pub fn borrow_mut(res: &Resources) -> WriteMouse {
    resources::borrow_mut(res)
  }

  pub fn button(&self, index: usize) -> bool {
    self.buttons.get(index).cloned().unwrap_or_default()
  }

  pub fn position(&self) -> Option<Point2<f32>> {
    self.position
  }

  fn set_button(&mut self, index: usize, value: bool) {
    let needed = index.saturating_sub(self.buttons.len());

    self.buttons.extend(iter::repeat(false).take(needed));

    let old_value = mem::replace(&mut self.buttons[index], value);

    if old_value != value {
      self.events.single_write(MouseEvent::ButtonChanged {
        button: MouseButton(index),
        value,
      });
    }
  }

  fn set_position(&mut self, value: Option<Point2<f32>>) {
    let old_value = mem::replace(&mut self.position, value);

    if old_value != value {
      self.events.single_write(MouseEvent::PositionChanged(value));
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MouseButton(pub usize);

#[derive(Debug)]
pub enum MouseEvent {
  PositionChanged(Option<Point2<f32>>),
  ButtonChanged { button: MouseButton, value: bool },
}

pub fn setup(engine: &mut Engine) {
  engine.resources.insert(Mouse::default());

  update::setup(engine);
}
