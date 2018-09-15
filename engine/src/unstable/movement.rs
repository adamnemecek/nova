// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Controlled;

#[derive(Default)]
pub struct Controller;

impl<'a> System<'a> for Controller {
  type SystemData = (
    Read<'a, core::Clock>,
    Read<'a, input::State>,
    ReadStorage<'a, Controlled>,
    WriteStorage<'a, stage::Position>,
    WriteStorage<'a, unstable::Character>,
  );

  fn run(&mut self, (clock, input, controlled, mut positions, mut characters): Self::SystemData) {
    let mut velocity = Vector3::<f32>::zeros();

    if input.is_pressed(input::Button::Up) {
      velocity.y -= 1.0;
    }

    if input.is_pressed(input::Button::Left) {
      velocity.x -= 1.0;
    }

    if input.is_pressed(input::Button::Down) {
      velocity.y += 1.0;
    }

    if input.is_pressed(input::Button::Right) {
      velocity.x += 1.0;
    }

    let idle = velocity == Vector3::zeros();

    if !idle {
      velocity.normalize_mut();
      velocity *= clock.delta_time as f32;
    }

    for (_, position, character) in (&controlled, &mut positions, &mut characters).join() {
      if idle {
        character.state = unstable::character::State::Idle;
      } else {
        let velocity = velocity * character.speed;

        character.state = unstable::character::State::Walking;

        position.x += velocity.x;
        position.y += velocity.y;
      }
    }
  }
}
