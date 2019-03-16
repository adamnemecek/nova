// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::ecs;
use nova_core::engine;

pub struct NodeContext<'a> {
  pub resources: &'a engine::Resources,
  pub entities: &'a ecs::Entities,
  pub entity: ecs::Entity,
  pub(crate) should_rebuild: &'a mut bool,
}

impl<'a> NodeContext<'a> {
  pub fn put_component<T: ecs::Component>(&self, component: T) {
    ecs::write_components(&self.resources)
      .insert(self.entity, component)
      .expect("The element entity is not alive.");
  }

  pub fn remove_component<T: ecs::Component>(&self) {
    ecs::write_components::<T>(&self.resources).remove(self.entity);
  }

  pub fn rebuild(&mut self) {
    *self.should_rebuild = true;
  }
}
