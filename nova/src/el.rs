// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod instance;
mod node;

use self::instance::{Instance, InstanceBox};
use crate::ecs;
use std::fmt;

pub use self::node::{node, Node};

pub trait Element: Send + Sync + fmt::Debug {
  type Props: Default + PartialEq + Send + Sync + fmt::Debug + 'static;

  fn new(props: &Self::Props) -> Self;

  fn on_prop_change(&mut self, _props: &Self::Props) -> RebuildNeeded {
    RebuildNeeded::Yes
  }

  fn build(&mut self, _props: &Self::Props) -> Node {
    Node::default()
  }
}

#[derive(Debug)]
pub enum RebuildNeeded {
  No,
  Yes,
}

#[derive(Debug)]
struct Mount {
  instance: Option<InstanceBox>,
  children: Vec<ecs::Entity>,
}
