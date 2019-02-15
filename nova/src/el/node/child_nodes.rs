// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Content, Node};
use crate::ecs;
use std::slice;

pub struct ChildNodes<'a> {
  pub(in crate::el) entities: slice::Iter<'a, ecs::Entity>,
}

impl<'a> Iterator for ChildNodes<'a> {
  type Item = Node;

  fn next(&mut self) -> Option<Node> {
    self.entities.next().map(|e| Node(Content::Entity(*e)))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.entities.size_hint()
  }
}

impl<'a> ExactSizeIterator for ChildNodes<'a> {}
