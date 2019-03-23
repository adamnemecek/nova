// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::storage::*;

use crate::collections::HashMap;
use hibitset::BitSetLike;
use specs::world::Index;

pub struct HashMapStorage<T>(HashMap<Index, T>);

impl<T> UnprotectedStorage<T> for HashMapStorage<T> {
  unsafe fn get(&self, id: Index) -> &T {
    &self.0[&id]
  }

  unsafe fn get_mut(&mut self, id: Index) -> &mut T {
    self.0.get_mut(&id).unwrap()
  }

  unsafe fn insert(&mut self, id: Index, v: T) {
    self.0.insert(id, v);
  }

  unsafe fn remove(&mut self, id: Index) -> T {
    self.0.remove(&id).unwrap()
  }

  unsafe fn clean<B>(&mut self, _has: B)
  where
    B: BitSetLike,
  {
    // not relevant
  }
}

impl<T> Default for HashMapStorage<T> {
  fn default() -> Self {
    Self(HashMap::new())
  }
}
