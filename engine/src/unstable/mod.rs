// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod movement {
  use prelude::*;

  pub fn setup<'a, 'b>(core: &mut Core, dispatch: &mut DispatcherBuilder<'a, 'b>) {
    core.world.register::<Controlled>();

    dispatch.add(Controller, "unstable::movement::Controller", &[]);
  }

  #[derive(Component)]
  #[storage(BTreeStorage)]
  pub struct Controlled {
    pub speed: f32,
  }

  impl Default for Controlled {
    fn default() -> Self {
      Controlled { speed: 1.0 }
    }
  }

  #[derive(Default)]
  pub struct Controller;

  impl<'a> System<'a> for Controller {
    type SystemData = (
      Read<'a, core::Clock>,
      Read<'a, input::State>,
      ReadStorage<'a, Controlled>,
      WriteStorage<'a, stage::Position>,
    );

    fn run(&mut self, (clock, input, controlled, mut positions): Self::SystemData) {
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

      if velocity == Vector3::zeros() {
        return;
      }

      velocity.normalize_mut();
      velocity *= clock.delta_time as f32;

      for (controlled, position) in (&controlled, &mut positions).join() {
        let velocity = velocity * controlled.speed;

        position.x += velocity.x;
        position.y += velocity.y;
      }
    }
  }
}
