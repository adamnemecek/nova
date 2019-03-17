// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::messages::{self, MessageQueue};
use nova_core::ecs;
use nova_core::engine;
use std::any::{Any, TypeId};

pub struct NodeContext<'a> {
  pub resources: &'a engine::Resources,
  pub entities: &'a ecs::Entities,
  pub entity: ecs::Entity,
  pub parent: Option<ecs::Entity>,
  pub(crate) should_rebuild: &'a mut bool,
  pub(crate) message_queue: &'a mut MessageQueue,
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

  pub fn dispatch<M>(&self, message: M)
  where
    M: Any + Send + Sync,
  {
    if let Some(parent) = self.parent {
      messages::write(self.resources).send(parent, message);
    }
  }

  pub(crate) fn subscribe(&mut self, type_id: TypeId) {
    self.message_queue.add_subscription(self.entity, type_id);
  }

  pub(crate) fn unsubscribe(&mut self, type_id: TypeId) {
    self.message_queue.remove_subscription(self.entity, type_id);
  }
}
