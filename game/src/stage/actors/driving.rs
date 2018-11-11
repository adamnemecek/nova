//! The `driving` module moves actors around the stage based on player input.
//!
//! It is currently the simplest possible solution and will likely need to be
//! completely changed in the near future.

use super::{Actor, Mode};
use crate::prelude::*;
use crate::stage::objects::Object;
use crate::stage::Velocity;
use nova::input::{Button, Input};

/// Component that indicates an actor is being driven.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Driven;

/// System that moves actors and adjusts their animation accordingly.
pub struct Driver;

impl<'a> System<'a> for Driver {
  type SystemData = (
    ReadResource<'a, Input>,
    ReadStorage<'a, Driven>,
    WriteStorage<'a, Actor>,
    WriteStorage<'a, Object>,
    WriteStorage<'a, Velocity>,
  );

  fn run(&mut self, (input, driven, mut actors, mut objects, mut velocities): Self::SystemData) {
    for (_, actor, object, velocity) in (&driven, &mut actors, &mut objects, &mut velocities).join()
    {
      let mut vector = Vector3::<f32>::zeros();

      if input.is_pressed(Button::Up) {
        vector.y -= 1.0;
      }

      if input.is_pressed(Button::Left) {
        vector.x -= 1.0;
      }

      if input.is_pressed(Button::Down) {
        vector.y += 1.0;
      }

      if input.is_pressed(Button::Right) {
        vector.x += 1.0;
      }

      if vector == Vector3::zeros() {
        actor.mode = Mode::Idle;
      } else {
        actor.mode = Mode::Walk;

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
  engine::add_system(ctx, Driver, "stage::actors::driving::Driver", &[]);
}

/// Sets the given entity to be driven by input.
pub fn drive(ctx: &mut engine::Context, entity: Entity) {
  let mut driven = engine::fetch_storage_mut::<Driven>(ctx);

  driven
    .insert(entity, Driven)
    .expect("could not insert Driven component");
}
