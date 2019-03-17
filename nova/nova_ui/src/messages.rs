// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::nodes::{self, NodeContext};
use crossbeam::queue::SegQueue;
use nova_core::collections::{FnvHashMap, FnvHashSet};
use nova_core::ecs;
use nova_core::engine::{Engine, Resources};
use std::any::{Any, TypeId};
use std::iter;
use std::sync::Arc;

pub type WriteMessages<'a> = ecs::ReadResource<'a, MessageQueue>;
pub type MessagePayload = Arc<dyn Any + Send + Sync>;

#[derive(Debug)]
pub struct Message {
  pub target: ecs::Entity,
  pub payload: MessagePayload,
}

#[derive(Debug, Default)]
pub struct MessageQueue {
  pub(crate) entries: SegQueue<Message>,
  pub(crate) subscriptions: FnvHashMap<TypeId, FnvHashSet<ecs::Entity>>,
}

impl MessageQueue {
  pub fn broadcast<M>(&self, payload: M) -> &Self
  where
    M: Any + Send + Sync + Clone,
  {
    let targets = match self.subscriptions.get(&TypeId::of::<M>()) {
      Some(targets) => targets,
      _ => return self,
    };

    for (target, payload) in targets.iter().cloned().zip(iter::repeat(payload)) {
      self.send(target, payload);
    }

    self
  }

  pub fn send<M>(&self, target: ecs::Entity, payload: M) -> &Self
  where
    M: Any + Send + Sync,
  {
    self.entries.push(Message {
      target,
      payload: Arc::new(payload),
    });

    self
  }

  pub(crate) fn next(&self) -> Option<Message> {
    self.entries.pop().ok()
  }

  pub(crate) fn add_subscription(&mut self, target: ecs::Entity, type_id: TypeId) {
    self
      .subscriptions
      .entry(type_id)
      .or_default()
      .insert(target);
  }

  pub(crate) fn remove_subscription(&mut self, target: ecs::Entity, type_id: TypeId) {
    if let Some(subscriptions) = self.subscriptions.get_mut(&type_id) {
      subscriptions.remove(&target);
    }
  }
}

pub fn setup(engine: &mut Engine) {
  engine.res.entry().or_insert_with(MessageQueue::default);
}

pub fn write(res: &Resources) -> WriteMessages {
  ecs::SystemData::fetch(res)
}

pub fn deliver(res: &Resources) {
  let message_queue = &mut res.fetch_mut::<MessageQueue>();
  let entities = &ecs::entities::read(res);
  let mut nodes = nodes::write(res);

  while let Some(message) = message_queue.next() {
    let node = match nodes.get_mut(message.target) {
      Some(node) => node,
      None => continue,
    };

    let result = node.element.on_message(
      NodeContext {
        resources: res,
        entities,
        message_queue,
        entity: message.target,
        parent: node.parent,
        should_rebuild: &mut node.should_rebuild,
      },
      message.payload,
    );

    if let (Err(payload), Some(parent)) = (result, node.parent) {
      message_queue.entries.push(Message {
        target: parent,
        payload,
      });
    }
  }
}
