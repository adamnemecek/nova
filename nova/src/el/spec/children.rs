// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Content, Spec};
use crate::ecs;
use std::slice;

pub struct Children<'a> {
  pub(in crate::el) entities: slice::Iter<'a, ecs::Entity>,
}

impl<'a> Iterator for Children<'a> {
  type Item = Spec;

  fn next(&mut self) -> Option<Spec> {
    self.entities.next().map(|e| Spec(Content::Entity(*e)))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.entities.size_hint()
  }
}

impl<'a> ExactSizeIterator for Children<'a> {}
