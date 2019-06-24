// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::controls::ControlBinding;
use nova_core::collections::HashSet;
use std::f32;

#[derive(Debug)]
pub struct Control {
  pub name: String,
  pub value: f32,
  pub is_pressed: bool,
  pub bindings: HashSet<ControlBinding>,
  pub negative_bindings: HashSet<ControlBinding>,
}

impl Control {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn value(&self) -> f32 {
    self.value
  }

  pub fn is_pressed(&self) -> bool {
    self.is_pressed
  }
}
