// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod common;
pub mod spec;

mod context;
mod element;
mod hierarchy;
mod instance;
mod message;

pub use self::context::Context;
pub use self::element::{Element, ShouldRebuild};
pub use self::hierarchy::Hierarchy;
pub use self::message::{Message, MessageComposer, MessageQueue};
pub use self::spec::{spec, Spec};

use self::instance::Instance;
use crate::ecs;
use crate::Engine;

pub fn setup(engine: &mut Engine) {
  engine.resources_mut().insert(Hierarchy::new());
  engine.resources_mut().insert(MessageQueue::new());

  ecs::register::<hierarchy::Node>(engine.resources_mut());

  common::setup(engine);
}
