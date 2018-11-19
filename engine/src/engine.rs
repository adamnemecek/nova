// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Container for all ECS resources including entities and components.
#[derive(Default)]
pub struct Engine {
  pub(crate) world: specs::World,
}

impl Engine {
  /// Creates a new engine instance.
  pub fn new() -> Self {
    let world = specs::World::new();

    Engine { world }
  }

  /// Maintains engine state. Should be called at the end of each frame or
  /// update.
  pub fn maintain(&mut self) {
    self.world.maintain();
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
