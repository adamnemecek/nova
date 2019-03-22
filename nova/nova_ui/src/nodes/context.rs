// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::messages::MessageQueue;
use nova_core::components::{self, Component};
use nova_core::entities::{Entities, Entity};
use nova_core::resources::Resources;
use std::any::{Any, TypeId};

pub struct NodeContext<'a> {
  pub resources: &'a Resources,
  pub entities: &'a Entities<'a>,
  pub entity: Entity,
  pub parent: Option<Entity>,
  pub(crate) messages: &'a mut MessageQueue,
  pub(crate) should_rebuild: &'a mut bool,
}

impl<'a> NodeContext<'a> {
  pub fn put_component<T: Component>(&self, component: T) {
    components::borrow_mut(&self.resources)
      .insert(self.entity, component)
      .expect("The element entity is not alive.");
  }

  pub fn remove_component<T: Component>(&self) {
    components::borrow_mut::<T>(&self.resources).remove(self.entity);
  }

  pub fn rebuild(&mut self) {
    *self.should_rebuild = true;
  }

  pub fn dispatch<M>(&self, message: M)
  where
    M: Any + Send + Sync,
  {
    if let Some(parent) = self.parent {
      self.messages.send(parent, message);
    }
  }

  pub(crate) fn subscribe(&mut self, type_id: TypeId) {
    self.messages.add_subscription(self.entity, type_id);
  }

  pub(crate) fn unsubscribe(&mut self, type_id: TypeId) {
    self.messages.remove_subscription(self.entity, type_id);
  }
}
