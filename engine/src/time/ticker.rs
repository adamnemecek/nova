// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Source, Time};
use crate::ecs;

#[derive(Debug)]
pub struct Ticker<S: Source> {
  pub source: S,
}

impl<S: Source> Ticker<S> {
  pub fn new(source: S) -> Self {
    Ticker { source }
  }
}

impl<'a, S: Source> ecs::System<'a> for Ticker<S> {
  type SystemData = ecs::WriteResource<'a, Time>;

  fn setup(&mut self, res: &mut ecs::Resources) {
    ecs::ensure_resource::<Time>(res);
  }

  fn run(&mut self, mut time: ecs::WriteResource<'a, Time>) {
    time.tick(self.source.delta_time());
  }
}
