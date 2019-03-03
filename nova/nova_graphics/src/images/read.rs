// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Image, ImageId};
use nova_core::ecs;
use nova_core::ecs::derive::*;

#[derive(SystemData)]
pub struct ReadImages<'a>(ecs::ReadComponents<'a, Image>);

impl<'a> ReadImages<'a> {
  pub fn get(&self, id: impl Into<ImageId>) -> Option<&Image> {
    self.0.get(id.into().into())
  }
}
