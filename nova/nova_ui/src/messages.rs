// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::nodes::{self, NodeContext};
use crossbeam::queue::SegQueue;
use nova_core::ecs;
use nova_core::engine::{Engine, Resources};
use std::any::Any;
use std::sync::Arc;

pub type ReadMessages<'a> = ecs::ReadResource<'a, MessageQueue>;
pub type WriteMessages<'a> = ReadMessages<'a>;

pub type MessagePayload = Arc<dyn Any + Send + Sync>;

#[derive(Debug)]
pub struct Message {
  pub target: ecs::Entity,
  pub payload: MessagePayload,
}

#[derive(Debug, Default)]
pub struct MessageQueue {
  pub(crate) entries: SegQueue<Message>,
}

impl MessageQueue {
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
}

pub fn setup(engine: &mut Engine) {
  engine
    .resources_mut()
    .entry()
    .or_insert_with(MessageQueue::default);
}

pub fn read(res: &Resources) -> ReadMessages {
  ecs::SystemData::fetch(res)
}

pub fn write(res: &Resources) -> WriteMessages {
  ecs::SystemData::fetch(res)
}

pub fn deliver(res: &Resources) {
  let messages = write(res);
  let entities = ecs::entities::read(res);
  let mut nodes = nodes::write(res);

  while let Some(message) = messages.next() {
    let node = match nodes.get_mut(message.target) {
      Some(node) => node,
      None => continue,
    };

    let result = node.element.on_message(
      NodeContext {
        resources: res,
        entities: &entities,
        entity: message.target,
        parent: node.parent,
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
