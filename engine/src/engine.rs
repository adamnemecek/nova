// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::ecs;
use crate::log;

/// Container for all ECS resources including entities and components.
#[derive(Default)]
pub struct Engine {
  pub(crate) world: specs::World,
}

impl Engine {
  /// Creates a new engine instance.
  pub fn new() -> Self {
    let mut engine = Engine {
      world: specs::World::new(),
    };

    log::setup(&mut engine);

    engine
  }

  /// Gets whether or not the engine has a resource of type `T`.
  pub fn has_resource<T: ecs::Resource>(&mut self) -> bool {
    self.world.res.has_value::<T>()
  }

  /// Adds a resource to the engine instance. If the resource already existed,
  /// the old value is overwritten.
  pub fn put_resource(&mut self, resource: impl ecs::Resource) {
    self.world.res.insert(resource);
  }

  /// Checks that a resource of type `T` exists in the engine. If it does not,
  /// the [`Default`] value of `T` is added to the engine.
  ///
  /// This function returns a mutable reference to the new or existing resource.
  pub fn ensure_resource<T: ecs::Resource + Default>(&mut self) -> &mut T {
    if !self.has_resource::<T>() {
      self.put_resource(T::default());
    }

    self.get_resource_mut()
  }

  /// Fetches a reference to a resource in the engine instance.
  ///
  /// # Panics
  ///
  /// This function panics if the resource does not exist or is currently
  /// fetched mutably.
  pub fn fetch_resource<T: ecs::Resource>(&self) -> ecs::FetchResource<T> {
    self.world.res.fetch()
  }

  /// Fetches a mutable reference to a resource in the engine instance.
  ///
  /// # Panics
  ///
  /// This function panics if the resource does not exist or is already
  /// fetched mutably.
  pub fn fetch_resource_mut<T: ecs::Resource>(&self) -> ecs::FetchResourceMut<T> {
    self.world.res.fetch_mut()
  }

  /// Gets a mutable reference to a resource in an engine instance. This is more
  /// efficient than fetching a resource.
  ///
  /// # Panics
  ///
  /// This function panics if the resource does not exist.
  pub fn get_resource_mut<T: ecs::Resource>(&mut self) -> &mut T {
    self
      .world
      .res
      .get_mut()
      .expect("The specified resource does not exist.")
  }

  /// Performs deferred tasks and other engine maintenance. Should be called
  /// once per frame.
  pub fn maintain(&mut self) {
    self.world.maintain()
  }
}

// Implement conversions to and from references of equivalent types.
//
// These conversions are safe because they are all the same in memory.
impl AsMut<Engine> for specs::Resources {
  fn as_mut(&mut self) -> &mut Engine {
    unsafe { &mut *(self as *mut Self as *mut Engine) }
  }
}

impl AsMut<Engine> for specs::World {
  fn as_mut(&mut self) -> &mut Engine {
    unsafe { &mut *(self as *mut Self as *mut Engine) }
  }
}

impl AsMut<specs::Resources> for Engine {
  fn as_mut(&mut self) -> &mut specs::Resources {
    unsafe { &mut *(self as *mut Self as *mut specs::Resources) }
  }
}

impl AsMut<specs::World> for Engine {
  fn as_mut(&mut self) -> &mut specs::World {
    unsafe { &mut *(self as *mut Self as *mut specs::World) }
  }
}