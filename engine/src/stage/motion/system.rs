// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::{Position, Velocity};

pub struct MotionSystem;

impl<'a> System<'a> for MotionSystem {
  type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

  fn run(&mut self, (velocities, mut positions): Self::SystemData) {
    for (velocity, position) in (&velocities, &mut positions).join() {
      position.point += velocity.vector;
    }
  }
}
