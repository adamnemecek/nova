// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `ecs` module exposes a parallel ECS implementation based on [specs][1].
//!
//! [1]: https://github.com/slide-rs/specs

pub mod components;
pub mod entities;
pub mod resources;
pub mod storages;
pub mod systems;

pub use self::components::*;
pub use self::entities::*;
pub use self::resources::*;
pub use self::storages::*;
pub use self::systems::*;

/// Container for all ECS resources including entities and components.
pub struct Context {
  world: specs::World,
}

impl Context {
  /// Creates a new, empty context.
  pub fn new() -> Self {
    let world = specs::World::new();

    Context { world }
  }

  /// Updates the context by performing manitainence work such as destroying
  /// entities or removing old component data.
  ///
  /// For example, a typical game should call this function once per frame or
  /// game loop iteration.
  pub fn update(&mut self) {
    self.world.maintain();
  }
}

// Implement conversions to and from references to equivalent types.
//
// When using specs one encounters two different “global state” types:
// `specs::World` and `shred::Resources`. Although `World` is just a wrapper
// around `Resources`, it contains a lot of helper methods and functionality not
// available with just `Resources`.
//
// Nova introduces a wrapper for both, `ecs::Context`. Because `Context` just
// has `World` which just has `Resources`, all of these structs have the same
// in-memory representation and are safely interchangeable using pointer casts.
// With these conversions, nova can wrap other ECS types and methods to always
// use `Context` instead of sometimes `World` and sometimes `Resources`.
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
