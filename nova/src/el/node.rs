// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod child_nodes;
mod into_iter;

pub use self::child_nodes::ChildNodes;
pub use self::into_iter::IntoIter;

use super::{Element, Prototype};
use crate::ecs;
use std::iter;
use std::mem;
use std::vec;

#[derive(Debug)]
pub struct Node(Content);

#[derive(Debug)]
enum Content {
  List(Vec<Node>),
  Element(Prototype),
  Entity(ecs::Entity),
}

impl Node {
  pub(super) fn entity(&self) -> Option<ecs::Entity> {
    match &self.0 {
      Content::Entity(entity) => Some(*entity),
      _ => None,
    }
  }

  pub(super) fn into_element_prototype(self) -> Prototype {
    match self.0 {
      Content::Element(prototype) => prototype,
      Content::List(_) => panic!("Cannot convert a list node into an element prototype."),
      Content::Entity(_) => panic!("Cannot convert an entity node into an element prototype."),
    }
  }
}

impl From<()> for Node {
  fn from(_: ()) -> Self {
    empty()
  }
}

impl From<Option<Node>> for Node {
  fn from(node: Option<Node>) -> Self {
    match node {
      Some(node) => node,
      None => empty(),
    }
  }
}

impl<E: Element + 'static> From<E> for Node {
  fn from(element: E) -> Self {
    node(element, Vec::new())
  }
}

impl<'a> From<ChildNodes<'a>> for Node {
  fn from(children: ChildNodes<'a>) -> Self {
    list(children.collect())
  }
}

impl From<Vec<Node>> for Node {
  fn from(nodes: Vec<Node>) -> Self {
    list(nodes)
  }
}

impl iter::FromIterator<Node> for Node {
  fn from_iter<I: IntoIterator<Item = Node>>(iter: I) -> Self {
    list(iter.into_iter().collect::<Vec<_>>())
  }
}

impl From<Node> for Vec<Node> {
  fn from(node: Node) -> Self {
    match node {
      Node(Content::List(nodes)) => nodes,
      node => vec![node],
    }
  }
}

impl IntoIterator for Node {
  type Item = Node;
  type IntoIter = IntoIter;

  fn into_iter(self) -> IntoIter {
    match self.0 {
      Content::List(nodes) => IntoIter::List(nodes.into_iter()),
      content => IntoIter::Element(iter::once(Node(content))),
    }
  }
}

pub fn empty() -> Node {
  Node(Content::List(Vec::new()))
}

pub fn list(mut children: Vec<Node>) -> Node {
  // Flatten nested list nodes.
  let mut i = 0;

  while i < children.len() {
    if let Node(Content::List(_)) = children[i] {
      // Swap an empty node with the list node in the vec.
      //
      // This is more efficient than removing it, which would move all later
      // elements back one index.
      let mut child = empty();

      mem::swap(&mut children[i], &mut child);

      // Splice the list's children into the vec at its former position,
      // which will overwrite the empty node that was just swapped in.
      children.splice(i..=i, child.into_iter());
    }

    i += 1;
  }

  Node(Content::List(children))
}

pub fn node(element: impl Element + 'static, children: impl Into<Node>) -> Node {
  Node(Content::Element(Prototype::new(
    element,
    children.into().into(),
  )))
}
