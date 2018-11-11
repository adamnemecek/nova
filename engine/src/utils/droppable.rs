// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

pub struct Droppable<T> {
  value: Option<T>,
}

impl<T> Droppable<T> {
  pub const fn dropped() -> Self {
    Droppable { value: None }
  }

  pub fn is_dropped(&self) -> bool {
    self.value.is_none()
  }

  pub fn take(&mut self) -> Option<T> {
    self.value.take()
  }

  pub fn drop(&mut self) {
    self.value = None;
  }
}

impl<T> From<T> for Droppable<T> {
  fn from(value: T) -> Self {
    Droppable { value: Some(value) }
  }
}

impl<T> Default for Droppable<T> {
  fn default() -> Self {
    Droppable::dropped()
  }
}

impl<T> Deref for Droppable<T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.value.as_ref().expect("value is dropped")
  }
}

impl<T> DerefMut for Droppable<T> {
  fn deref_mut(&mut self) -> &mut T {
    self.value.as_mut().expect("value is dropped")
  }
}

impl<T> AsRef<T> for Droppable<T> {
  fn as_ref(&self) -> &T {
    self.deref()
  }
}
