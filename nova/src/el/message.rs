// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod composer;

pub use self::composer::MessageComposer;

use super::Element;
use crate::ecs;
use std::any::Any;
use std::fmt;

#[derive(Debug)]
pub struct Message {
  pub(crate) recipient: ecs::Entity,
  pub(crate) payload: Box<dyn Any + Send>,
}

pub trait Payload: Any + Send + fmt::Debug {}

impl<T: Any + Send + fmt::Debug> Payload for T {}
