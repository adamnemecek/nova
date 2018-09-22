// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `driving` module moves actors around the stage based on player input.
//!
//! It is currently the simplest possible solution and will likely need to be
//! completely changed in the near future.

use super::*;

/// Component that indicates an actor is being driven.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Driven;

/// System that moves actors and adjusts their animation accordingly.
pub struct Driver;

impl<'a> System<'a> for Driver {
  type SystemData = (
    ReadResource<'a, input::Input>,
    ReadStorage<'a, Driven>,
    WriteStorage<'a, Actor>,
    WriteStorage<'a, objects::Object>,
    WriteStorage<'a, Velocity>,
  );

  fn run(&mut self, (input, driven, mut actors, mut objects, mut velocities): Self::SystemData) {
    for (_, actor, object, velocity) in (&driven, &mut actors, &mut objects, &mut velocities).join()
    {
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

      if vector == Vector3::zeros() {
        actor.mode = stage::actors::Mode::Idle;
      } else {
        actor.mode = stage::actors::Mode::Walk;

        vector.normalize_mut();
        object.facing = vector;
        vector *= actor.template.walk_speed;
      }

      velocity.vector = vector;
    }
  }
}

/// Initializes actor driving for the given engine context.
pub fn init(ctx: &mut engine::Context) {
  engine::add_storage::<Driven>(ctx);
  engine::init::add_system_early(ctx, Driver, "stage::actors::driving::Driver", &[]);
}

/// Sets the given entity to be driven by input.
pub fn drive(ctx: &mut engine::Context, entity: Entity) {
  let mut driven = engine::fetch_storage_mut::<Driven>(ctx);

  driven
    .insert(entity, Driven)
    .expect("could not insert Driven component");
}
