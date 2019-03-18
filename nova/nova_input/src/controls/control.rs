// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::controls::ControlBinding;
use nova_core::collections::FnvHashSet;
use nova_core::SharedStr;
use std::f32;

#[derive(Debug)]
pub struct Control {
  pub(crate) name: SharedStr,
  pub(crate) value: f32,
  pub(crate) is_pressed: bool,
  pub(crate) bindings: FnvHashSet<ControlBinding>,
  pub(crate) negative_bindings: FnvHashSet<ControlBinding>,
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
