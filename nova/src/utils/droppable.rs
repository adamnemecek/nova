// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

/// A value that can be dropped at any time.
///
/// Similar to other programming languages with nullable references, using a
/// dropped value causes a panic.
///
/// This type is intended for fields of structures that must be manually dropped
/// but ownership normally could not be taken in the `drop()` function. For
/// example, graphics device resources.
pub struct Droppable<T> {
  value: Option<T>,
}

impl<T> Droppable<T> {
  /// Gets the representation of a dropped value.
  pub const fn dropped() -> Self {
    Droppable { value: None }
  }

  /// Gets whether or not the value has been dropped or taken. If this returns
  /// `true`, using the value will cause a panic.
  pub fn is_dropped(&self) -> bool {
    self.value.is_none()
  }

  /// Takes the value. Returns `None` if the value was already taken or dropped.
  pub fn take(&mut self) -> Option<T> {
    self.value.take()
  }
}

// Implement `From` to convert from a bare value.
impl<T> From<T> for Droppable<T> {
  fn from(value: T) -> Self {
    Droppable { value: Some(value) }
  }
}

// Implement `Default` to return a dropped value.
impl<T> Default for Droppable<T> {
  fn default() -> Self {
    Droppable::dropped()
  }
}

// Implement `DeRef` and `Borrow` to expose the inner value.
impl<T> Deref for Droppable<T> {
  type Target = T;

  fn deref(&self) -> &T {
    self
      .value
      .as_ref()
      .expect("The value has been dropped, taken, or was never set.")
  }
}

impl<T> Borrow<T> for Droppable<T> {
  fn borrow(&self) -> &T {
    self.deref()
  }
}

// Implement `DeRefMut` and `BorrowMut` to expose the inner value mutably.
impl<T> DerefMut for Droppable<T> {
  fn deref_mut(&mut self) -> &mut T {
    self
      .value
      .as_mut()
      .expect("The value has been dropped, taken, or was never set.")
  }
}

impl<T> BorrowMut<T> for Droppable<T> {
  fn borrow_mut(&mut self) -> &mut T {
    self.deref_mut()
  }
}
