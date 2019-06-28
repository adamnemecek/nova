// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A container for a value which is initially present but can be taken, after
/// which all accesses will panic.
///
/// This container is intended for struct fields with a value that must be
/// present until the struct is dropped but must also be owned to be properly
/// destroyed, such as a graphics device resource or an element of an object
/// pool.
pub struct Expect<T> {
  value: Option<T>,
}

impl<T> Expect<T> {
  /// Creates a new `Expect` with the given `value`.
  pub fn new(value: T) -> Self {
    Self { value: Some(value) }
  }

  /// Takes the value out of the `Expect`.
  ///
  /// # Panics
  ///
  /// Panics if the value has already been taken.
  pub fn take(&mut self) -> T {
    self
      .try_take()
      .expect("expected value has already been taken")
  }

  /// Takes the value out of the `Expect`, returning `None` if the value has
  /// already been taken.
  pub fn try_take(&mut self) -> Option<T> {
    self.value.take()
  }
}

// Implement dereference operations to access the value or panic if it has been
// taken.
impl<T> ops::Deref for Expect<T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.value.as_ref().expect("expected value has been taken")
  }
}

impl<T> ops::DerefMut for Expect<T> {
  fn deref_mut(&mut self) -> &mut T {
    self.value.as_mut().expect("expected value has been taken")
  }
}

// Implement `From` for easy conversions from regular values.
impl<T> From<T> for Expect<T> {
  fn from(value: T) -> Self {
    Expect { value: Some(value) }
  }
}
