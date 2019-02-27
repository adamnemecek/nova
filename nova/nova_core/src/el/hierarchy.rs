// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod node;
mod read;

pub(crate) use self::node::{Children, Node};
pub use self::read::ReadHierarchyNodes;

use super::context::NodeContext;
use super::{Element, Instance, Message, MessageQueue, ShouldRebuild, Spec};
use crate::ecs;
use crate::engine;
use crate::log;
use std::ops::RangeBounds;

#[derive(Debug)]
pub struct Hierarchy {
  log: log::Logger,
  pub roots: Vec<ecs::Entity>,
  pub sorted: Vec<ecs::Entity>,
  /// Resuable temporary storage for the stack of entities that need to be
  /// built.
  build_stack: Vec<ecs::Entity>,
  /// Resuable temporary storage for the stack of entities that need new specs
  /// applied.
  apply_stack: Vec<(ecs::Entity, Spec)>,
  /// Resuable temporary storage for the stack of entities that need to be
  /// deleted.
  delete_stack: Vec<ecs::Entity>,
}

impl Default for Hierarchy {
  fn default() -> Self {
    Self {
      log: log::Logger::new("nova::el::Hierarchy"),
      roots: Vec::new(),
      sorted: Vec::new(),
      build_stack: Vec::new(),
      apply_stack: Vec::new(),
      delete_stack: Vec::new(),
    }
  }
}

impl Hierarchy {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn roots<'a>(&'a self) -> impl Iterator<Item = ecs::Entity> + 'a {
    self.roots.iter().cloned()
  }

  pub fn sorted<'a>(&'a self) -> impl Iterator<Item = ecs::Entity> + 'a {
    self.sorted.iter().cloned()
  }

  pub fn add_element<E: Element + 'static>(&mut self, res: &engine::Resources, element: E) {
    let entities = res.fetch::<ecs::Entities>();
    let entity = entities.create();

    let ctx = NodeContext {
      resources: res,
      entities: &entities,
      entity,
      messages: &res.fetch::<MessageQueue>(),
    };

    let mut nodes = ecs::write_components::<Node>(res);

    let _ = nodes.insert(entity, Node::new(Instance::new(element, ctx)));

    self.roots.push(entity);
  }

  pub fn deliver_messages(&mut self, res: &engine::Resources) {
    let entities = res.fetch::<ecs::Entities>();
    let messages = res.fetch::<MessageQueue>();
    let mut nodes = ecs::write_components::<Node>(res);

    while let Some(Message { recipient, payload }) = messages.take() {
      match nodes.get_mut(recipient) {
        Some(node) => {
          let ctx = NodeContext {
            resources: res,
            entities: &entities,
            entity: recipient,
            messages: &messages,
          };

          match node.instance.on_message(payload, ctx) {
            Ok(ShouldRebuild(true)) => {
              node.needs_build = true;
            }

            Err(payload) => {
              self.log
                .debug("Message dropped.")
                .with("reason", "wrong type")
                .with("recipient", recipient.id())
                .with("payload", payload);
            }

            _ => {}
          };
        }

        None => {
          self.log
            .debug("Message dropped.")
            .with("reason", "no mounted element")
            .with("recipient", recipient.id())
            .with("payload", payload);
        }
      }
    }
  }

  pub fn build(&mut self, res: &engine::Resources) {
    let entities = res.fetch::<ecs::Entities>();
    let messages = res.fetch::<MessageQueue>();
    let mut nodes = ecs::write_components::<Node>(res);

    // Clear the sorted hierarchy which is about to change.
    self.sorted.clear();

    // Push each hierarchy root entity onto the stack in reverse order.
    for root in self.roots.iter().rev() {
      self.build_stack.push(*root);
    }

    // For each entity on the stack…
    while let Some(entity) = self.build_stack.pop() {
      // By now, all entities before this one in the hierarchy have been
      // visited, so add this entity to the sorted vec.
      self.sorted.push(entity);

      let mut was_rebuilt = false;

      if let Some(node) = nodes.get_mut(entity) {
        let ctx = NodeContext {
          resources: res,
          entities: &entities,
          entity,
          messages: &messages,
        };

        // Awake the element. Does nothing if already awake.
        node.instance.awake(ctx);

        // Rebuild the element if needed.
        if node.needs_build {
          let spec = node.instance.build(&node.spec_children, ctx);

          self.push_apply_children(ctx, spec, &mut node.real_children);

          was_rebuilt = true;
          node.needs_build = false;
        }

        // Push all children onto the build stack in reverse order so that
        // they are sorted into the hierarchy correctly.
        self.build_stack
          .extend(node.real_children.entities.iter().rev().cloned());
      }

      if was_rebuilt {
        self.apply(res, &entities, &messages, &mut nodes);
      }
    }

    self.delete(res, &entities, &messages, &mut nodes);
  }

  fn apply(
    &mut self,
    res: &engine::Resources,
    entities: &ecs::Entities,
    messages: &MessageQueue,
    nodes: &mut ecs::WriteComponents<Node>,
  ) {
    while let Some((entity, spec)) = self.apply_stack.pop() {
      let ctx = NodeContext {
        resources: res,
        entities,
        entity,
        messages,
      };

      let prototype = spec.into_element_prototype();
      let mut should_rebuild = ShouldRebuild(true);

      // Apply the prototype to the entity and get its element node.
      let node = match nodes.get_mut(entity) {
        Some(node) => {
          // If the node already exists, update its props.
          match node.instance.replace_element(prototype.element, ctx) {
            Ok(rebuild) => {
              should_rebuild = rebuild;
            }

            Err(element) => {
              // The nodeed instance has a different type of element, so
              // replace it with a new instance based on the prototype.
              node.instance.sleep(ctx);
              node.instance = (prototype.new)(element, ctx);
            }
          };

          node
        }

        None => {
          // If the node doesn't exist, node a new element instance based
          // on the prototype.
          nodes
            .insert(entity, Node::new((prototype.new)(prototype.element, ctx)))
            .expect("Could not create element node");

          nodes
            .get_mut(entity)
            .expect("Could not get newly created element node")
        }
      };

      node.needs_build =
        *self.push_apply_children(ctx, prototype.children, &mut node.spec_children)
          || node.needs_build
          || *should_rebuild;
    }
  }

  fn delete(
    &mut self,
    res: &engine::Resources,
    entities: &ecs::Entities,
    messages: &MessageQueue,
    nodes: &mut ecs::WriteComponents<Node>,
  ) {
    while let Some(entity) = self.delete_stack.pop() {
      let ctx = NodeContext {
        resources: res,
        entities,
        entity,
        messages,
      };

      if let Some(node) = nodes.get_mut(entity) {
        node.instance.sleep(ctx);

        self.push_delete_children(&mut node.spec_children, ..);
        self.push_delete_children(&mut node.real_children, ..);
      }

      nodes.remove(entity);

      let _ = entities.delete(entity);
    }
  }

  fn push_apply_children<I>(
    &mut self,
    ctx: NodeContext,
    spec: I,
    children: &mut Children,
  ) -> ShouldRebuild
  where
    I: IntoIterator<Item = Spec>,
    I::IntoIter: ExactSizeIterator,
  {
    let specs = spec.into_iter();

    let current_len = children.entities.len();
    let new_len = specs.len();

    // Flag any extra children for deletion.
    if new_len < current_len && current_len > 0 {
      self.push_delete_children(children, new_len..);
    }

    for (i, child) in specs.enumerate() {
      if let Some(child_entity) = child.entity() {
        // The child is an entity so reference it directly.

        if i < children.entities.len() {
          let existing = children.entities[i];

          if !children.references.remove(&existing) {
            self.delete_stack.push(existing);
          }

          children.entities[i] = child_entity;
        } else {
          children.entities.push(child_entity);
        }

        // Flag this child as a reference entity so it isn't deleted or
        // overwritten on rebuild.
        children.references.insert(child_entity);
      } else {
        // The child is an element so add it to the stack for application.

        if i < children.entities.len() {
          let existing = children.entities[i];

          // If the existing element was a reference, replace it with a new
          // entity.
          if children.references.remove(&existing) {
            children.entities[i] = ctx.entities.create();
          }
        } else {
          children.entities.push(ctx.entities.create());
        }

        self.apply_stack.push((children.entities[i], child));
      }
    }

    // Rebuild is only needed if the number of children changes.
    ShouldRebuild(current_len != new_len)
  }

  pub(crate) fn push_delete_children(
    &mut self,
    children: &mut Children,
    range: impl RangeBounds<usize>,
  ) {
    for entity in children.entities.drain(range) {
      if !children.references.remove(&entity) {
        self.delete_stack.push(entity);
      }
    }
  }
}
