// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::ecs;
use nova_core::math::Rect;
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ScreenRect(pub(crate) Rect<f32>);

impl Deref for ScreenRect {
  type Target = Rect<f32>;

  fn deref(&self) -> &Rect<f32> {
    &self.0
  }
}

impl DerefMut for ScreenRect {
  fn deref_mut(&mut self) -> &mut Rect<f32> {
    &mut self.0
  }
}

impl ecs::Component for ScreenRect {
  type Storage = ecs::HashMapStorage<Self>;
}
