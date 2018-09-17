// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub mod actor;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct InputControlled;

#[derive(Default)]
pub struct MotionInputSystem;

impl<'a> System<'a> for MotionInputSystem {
  type SystemData = (
    Read<'a, core::Clock>,
    Read<'a, input::Input>,
    ReadStorage<'a, InputControlled>,
    WriteStorage<'a, stage::Velocity>,
  );

  fn run(&mut self, (clock, input, controlled, mut velocities): Self::SystemData) {
    for (_, velocity) in (&controlled, &mut velocities).join() {
      let mut vector = Vector3::<f32>::zeros();

      if input.is_pressed(input::Button::Up) {
        vector.y -= 1.0;
      }

      if input.is_pressed(input::Button::Left) {
        vector.x -= 1.0;
      }

      if input.is_pressed(input::Button::Down) {
        vector.y += 1.0;
      }

      if input.is_pressed(input::Button::Right) {
        vector.x += 1.0;
      }

      if vector != Vector3::zeros() {
        vector.normalize_mut();
        vector *= (clock.delta_time * 64.0) as f32;
      }

      velocity.vector = vector;
    }
  }
}

/// Sets up unstable components, resources, and systems.
pub fn setup<'a, 'b>(core: &mut Core, dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<InputControlled>();

  dispatch.add(MotionInputSystem, "unstable::MotionInputSystem", &[]);
}
