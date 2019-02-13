// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Element, Prototype};
use crate::ecs;
use std::iter;
use std::mem;
use std::slice;
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

pub fn node<E: Element + 'static>(element: E, children: Vec<Node>) -> Node {
  Node(Content::Element(Prototype::new(element, children)))
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

pub enum IntoIter {
  Element(iter::Once<Node>),
  List(vec::IntoIter<Node>),
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

pub struct ChildNodes<'a> {
  pub(super) entities: slice::Iter<'a, ecs::Entity>,
}

impl<'a> Iterator for ChildNodes<'a> {
  type Item = Node;

  fn next(&mut self) -> Option<Node> {
    self.entities.next().map(|e| Node(Content::Entity(*e)))
  }
}
