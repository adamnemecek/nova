use super::{Array, SmallVec};
use std::iter;

pub struct Chain<A: Array> {
  items: SmallVec<A>,
  index: usize,
}

impl<A: Array> Chain<A> {
  pub fn allocate(create: impl FnMut() -> A::Item) -> Self {
    Chain {
      items: iter::repeat_with(create).take(A::size()).collect(),
      index: 0,
    }
  }

  pub fn next(&mut self) -> &mut A::Item {
    let index = self.index;

    self.index += 1;
    self.index %= self.items.len();

    &mut self.items[index]
  }
}
