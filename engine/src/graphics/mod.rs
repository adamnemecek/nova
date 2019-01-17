// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod backend;

pub use self::backend::*;
pub use gfx_hal::Instance as BackendInstanceExt;

use crate::ecs;

type BackendDevice = <Backend as gfx_hal::Backend>::Device;

pub struct Instance {
  instance: BackendInstance,
}

impl Instance {
  pub fn new() -> Self {
    Instance {
      instance: BackendInstance::create("nova", 1),
    }
  }
}

impl Default for Instance {
  fn default() -> Self {
    Instance::new()
  }
}

pub struct GraphicsSystem {
  device: Option<BackendDevice>,
}

impl<'a> ecs::System<'a> for GraphicsSystem {
  type SystemData = ecs::ReadResource<'a, Instance>;

  fn setup(&mut self, res: &mut ecs::Resources) {
    let instance = res.entry().or_insert_with(Instance::default);
    let adapters = instance.enumerate_adapters();
  }
}
