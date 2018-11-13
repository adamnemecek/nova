// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod components;
pub mod resources;
pub mod storages;
pub mod systems;

pub use self::components::*;
pub use self::resources::*;
pub use self::storages::*;
pub use self::systems::*;
pub use specs::Entity;

pub mod derive {
  pub use super::storages::*;
  pub use super::Component;
  pub use specs_derive::*;
}

pub struct Context {
  world: specs::World,
}

impl Context {
  pub fn new() -> Self {
    let world = specs::World::new();

    Context { world }
  }
}

// Implement conversions to and from references to equivalent types.
impl AsMut<Context> for specs::Resources {
  fn as_mut(&mut self) -> &mut Context {
    unsafe { &mut *(self as *mut Self as *mut Context) }
  }
}

impl AsMut<Context> for specs::World {
  fn as_mut(&mut self) -> &mut Context {
    unsafe { &mut *(self as *mut Self as *mut Context) }
  }
}

impl AsMut<specs::Resources> for Context {
  fn as_mut(&mut self) -> &mut specs::Resources {
    unsafe { &mut *(self as *mut Self as *mut specs::Resources) }
  }
}

impl AsMut<specs::World> for Context {
  fn as_mut(&mut self) -> &mut specs::World {
    unsafe { &mut *(self as *mut Self as *mut specs::World) }
  }
}
