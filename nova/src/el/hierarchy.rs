// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod context;
mod node;

use super::{Element, Instance, Message, MessageQueue, ShouldRebuild, Spec};
use crate::ecs;
use crate::engine;
use crate::log;

pub use self::context::Context;
pub(crate) use self::node::{Children, Node};

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

  pub fn add_element<E: Element + 'static>(&mut self, res: &engine::Resources, element: E) {
    let entities = res.fetch::<ecs::Entities>();
    let mut nodes = ecs::write_components::<Node>(res);

    self.roots.push(
      entities
        .build_entity()
        .with(Node::new(Instance::new(element)), &mut nodes)
        .build(),
    );
  }

  pub fn deliver_messages(&mut self, res: &engine::Resources) {
    let entities = res.fetch::<ecs::Entities>();
    let messages = res.fetch::<MessageQueue>();
    let mut nodes = ecs::write_components::<Node>(res);

    while let Some(Message { recipient, payload }) = messages.take() {
      match nodes.get_mut(recipient) {
        Some(node) => {
          let ctx = &mut Context {
            entity: recipient,
            hierarchy: self,
            resources: res,
            entities: &entities,
            messages: &messages,
          };

          match node.instance.on_message(payload, ctx) {
            Ok(ShouldRebuild(true)) => {
              node.needs_build = true;
            }

            Err(payload) => {
              self
                .log
                .debug("Message dropped.")
                .with("reason", "wrong type")
                .with("recipient", recipient.id())
                .with("payload", payload);
            }

            _ => {}
          };
        }

        None => {
          self
            .log
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
        let mut ctx = Context {
          entity,
          hierarchy: self,
          resources: res,
          entities: &entities,
          messages: &messages,
        };

        // Awake the element. Does nothing if already awake.
        node.instance.awake(&mut ctx);

        // Rebuild the element if needed.
        if node.needs_build {
          let spec = node.instance.build(&node.real_children, &mut ctx);

          ctx.push_apply_children(spec, &mut node.real_children);

          was_rebuilt = true;
          node.needs_build = false;
        }

        // Push all children onto the build stack in reverse order so that
        // they are sorted into the hierarchy correctly.
        self
          .build_stack
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
      let ctx = &mut Context {
        hierarchy: self,
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
              node.instance = (prototype.new)(element);
            }
          };

          node
        }

        None => {
          // If the node doesn't exist, node a new element instance based
          // on the prototype.
          nodes
            .insert(entity, Node::new((prototype.new)(prototype.element)))
            .expect("Could not create element node");

          nodes
            .get_mut(entity)
            .expect("Could not get newly created element node")
        }
      };

      if let ShouldRebuild(true) =
        ctx.push_apply_children(prototype.children, &mut node.spec_children)
      {
        *should_rebuild = true;
      }

      node.needs_build = node.needs_build || *should_rebuild;
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
      let ctx = &mut Context {
        hierarchy: self,
        resources: res,
        entities,
        entity,
        messages,
      };

      if let Some(node) = nodes.get_mut(entity) {
        node.instance.sleep(ctx);

        ctx.push_delete_children(&mut node.spec_children, ..);
        ctx.push_delete_children(&mut node.real_children, ..);
      }

      nodes.remove(entity);

      let _ = entities.delete(entity);
    }
  }
}
