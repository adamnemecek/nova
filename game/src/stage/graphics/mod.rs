use crate::prelude::*;

pub mod actors;
pub mod objects;

mod camera;

pub use self::camera::*;

/// Initializes stage graphics for the given engine context.
pub fn init(ctx: &mut engine::Context) {
  engine::add_resource(ctx, Camera::default());

  objects::init(ctx);
  actors::init(ctx);
}
