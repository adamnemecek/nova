// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Content, Node};
use std::iter;
use std::vec;

pub enum IntoIter {
  Element(iter::Once<Node>),
  List(vec::IntoIter<Node>),
}

impl From<Node> for IntoIter {
  fn from(node: Node) -> Self {
    match node.0 {
      Content::List(nodes) => IntoIter::List(nodes.into_iter()),
      content => IntoIter::Element(iter::once(Node(content))),
    }
  }
}

impl Iterator for IntoIter {
  type Item = Node;

  fn next(&mut self) -> Option<Node> {
    match self {
      IntoIter::Element(iter) => iter.next(),
      IntoIter::List(iter) => iter.next(),
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    match self {
      IntoIter::Element(iter) => iter.size_hint(),
      IntoIter::List(iter) => iter.size_hint(),
    }
  }
}

impl ExactSizeIterator for IntoIter {}
