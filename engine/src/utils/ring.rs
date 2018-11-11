// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// A fixed size “ring” of items that yields a reference to each item in a
/// looping sequence.
///
/// This is structure is for when a fixed size pool of resources are needed
/// where by the time all the resources have been used, the first resource is
/// now available to use again. This must be externally enforced.
pub struct Ring<T> {
  /// Items in the ring.
  items: Vec<T>,
  /// Current index in the ring.
  index: usize,
}

impl<T> Ring<T> {
  /// Creates a new ring of the given size. The `create` function will be called
  /// once for each item in the ring with its index and should return the value
  /// for that item.
  pub fn new(size: usize, create: impl FnMut(usize) -> T) -> Self {
    Ring {
      items: (0..size).map(create).collect(),
      index: 0,
    }
  }

  /// Gets a reference to the current item.
  pub fn current(&self) -> &T {
    &self.items[self.index]
  }

  /// Gets a mutable reference to the current item.
  pub fn current_mut(&mut self) -> &mut T {
    &mut self.items[self.index]
  }

  /// Moves to the next item and returns a mutable reference to it.
  pub fn next(&mut self) -> &mut T {
    let index = self.index;

    self.index += 1;
    self.index %= self.items.len();

    &mut self.items[index]
  }
}
