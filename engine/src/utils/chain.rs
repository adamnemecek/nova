// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub struct Chain<T> {
  items: Vec<T>,
  index: usize,
}

impl<T> Chain<T> {
  pub fn allocate(size: usize, create: impl FnMut(usize) -> T) -> Self {
    Chain {
      items: (0..size).map(create).collect(),
      index: 0,
    }
  }

  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut(self)
  }

  pub fn current(&self) -> &T {
    &self.items[self.index]
  }

  pub fn current_mut(&mut self) -> &mut T {
    &mut self.items[self.index]
  }

  pub fn next(&mut self) -> &mut T {
    let index = self.index;

    self.index += 1;
    self.index %= self.items.len();

    &mut self.items[index]
  }
}

pub struct IterMut<'a, T>(&'a mut Chain<T>);

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    let item = self.0.next() as *mut T;

    Some(unsafe { &mut *item })
  }
}

impl<'a, T> IntoIterator for &'a mut Chain<T> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}
