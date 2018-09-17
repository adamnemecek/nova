// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub mod camera;
pub mod direction;
pub mod motion;
pub mod object;
pub mod renderer;

pub use self::camera::Camera;
pub use self::direction::CompassDirection;
pub use self::motion::{MotionSystem, Position, Velocity};
pub use self::object::{Object, ObjectTemplate};
pub use self::renderer::{Render, Renderer};

/// Sets up stage components, resources, and systems.
pub fn setup<'a, 'b>(core: &mut Core, dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<Position>();
  core.world.register::<Velocity>();
  core.world.register::<Render>();
  core.world.register::<Object>();
  core.world.register::<object::animation::Animated>();

  core.world.add_resource(Camera::default());

  dispatch.add(MotionSystem, "stage::MotionSystem", &[]);
  dispatch.add(object::AnimationSystem, "object::AnimationSystem", &[]);
}

pub fn build_object<'a>(
  template: Arc<ObjectTemplate>,
  builder: EntityBuilder<'a>,
) -> EntityBuilder<'a> {
  builder
    .with(graphics::Sprite::new(template.atlas.clone()))
    .with(Position::default())
    .with(Velocity::default())
    .with(Render)
    .with(object::animation::Animated::default())
    .with(Object {
      template: template.clone(),
      facing: Vector3::y(),
    })
}
