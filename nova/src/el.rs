// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod node;

mod hierarchy;
mod instance;
mod mount;
mod prototype;

use self::instance::InstanceBox;
use self::mount::Mount;
use self::prototype::Prototype;
use crate::ecs;
use std::fmt;

pub use self::hierarchy::BuildHierarchy;
pub use self::node::{node, Node};

pub trait Element: Send + Sync + fmt::Debug {
  type Props: Default + PartialEq + Send + Sync + fmt::Debug + 'static;

  fn new(props: &Self::Props) -> Self;

  fn on_prop_change(&mut self, _props: &Self::Props) -> ShouldRebuild {
    ShouldRebuild::Yes
  }

  fn build(&mut self, _props: &Self::Props) -> Node {
    node::empty()
  }
}

#[derive(Debug)]
pub enum ShouldRebuild {
  No,
  Yes,
}

#[derive(Default)]
pub struct RebuildRequired;

impl ecs::Component for RebuildRequired {
  type Storage = ecs::NullStorage<Self>;
}
