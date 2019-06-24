// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::nodes::{self, NodeContext};
use crossbeam::queue::SegQueue;
use nova_core::collections::{HashMap, HashSet};
use nova_core::engine::Engine;
use nova_core::entities::{self, Entity};
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::any::{Any, TypeId};
use std::iter;
use std::sync::Arc;

pub type Messages<'a> = ReadResource<'a, MessageQueue>;
pub type MessagePayload = Arc<dyn Any + Send + Sync>;

#[derive(Debug)]
pub struct Message {
  pub target: Entity,
  pub payload: MessagePayload,
}

#[derive(Debug, Default)]
pub struct MessageQueue {
  pub entries: SegQueue<Message>,
  pub subscriptions: HashMap<TypeId, HashSet<Entity>>,
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

  pub fn send<M>(&self, target: Entity, payload: M) -> &Self
  where
    M: Any + Send + Sync,
  {
    self.entries.push(Message {
      target,
      payload: Arc::new(payload),
    });

    self
  }

  pub fn next(&self) -> Option<Message> {
    self.entries.pop().ok()
  }

  pub fn add_subscription(&mut self, target: Entity, type_id: TypeId) {
    self
      .subscriptions
      .entry(type_id)
      .or_default()
      .insert(target);
  }

  pub fn remove_subscription(&mut self, target: Entity, type_id: TypeId) {
    if let Some(subscriptions) = self.subscriptions.get_mut(&type_id) {
      subscriptions.remove(&target);
    }
  }
}

pub fn set_up(engine: &mut Engine) {
  engine
    .resources
    .entry()
    .or_insert_with(MessageQueue::default);
}

pub fn borrow(res: &Resources) -> Messages {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteResource<MessageQueue> {
  resources::borrow_mut(res)
}

pub fn deliver(resources: &Resources) {
  let entities = entities::borrow(resources);
  let mut nodes = nodes::borrow_mut(resources);

  let mut messages = borrow_mut(resources);

  while let Some(message) = messages.next() {
    let node = match nodes.get_mut(message.target) {
      Some(node) => node,
      None => continue,
    };

    let result = node.element.on_message(
      NodeContext {
        resources,
        entities: &entities,
        entity: message.target,
        parent: node.parent,
        messages: &mut messages,
        should_rebuild: &mut node.should_rebuild,
      },
      message.payload,
    );

    if let (Err(payload), Some(parent)) = (result, node.parent) {
      messages.entries.push(Message {
        target: parent,
        payload,
      });
    }
  }
}
