// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::Clock;

#[derive(Default)]
pub struct Engine;

impl Engine {
  pub fn new(world: &mut World) -> Self {
    world.add_resource(Clock::default());

    Engine
  }

  pub fn update(&mut self, world: &World, platform: &mut platform::Engine) {
    // Update ggez's timer.
    platform.ctx.timer_context.tick();

    // Update the clock resource.
    let mut clock = world.write_resource::<Clock>();

    clock.tick += 1;
    clock.delta_time = ggez::timer::duration_to_f64(ggez::timer::delta(&mut platform.ctx));
    clock.time += clock.delta_time;
  }
}
