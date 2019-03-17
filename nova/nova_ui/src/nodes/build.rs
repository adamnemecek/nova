// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::messages::MessageQueue;
use crate::nodes::{self, ChildNodes, NodeContext, WriteNodes};
use crate::specs::{ChildSpecs, Spec};
use nova_core::ecs;
use nova_core::engine::Engine;
use std::mem;
use std::ops::RangeBounds;

#[derive(Debug, Default)]
struct BuildState {
  /// Resuable temporary storage for the stack of entities that need to be
  /// built.
  build_stack: Vec<ecs::Entity>,
  /// Resuable temporary storage for the stack of entities that need new specs
  /// applied.
  apply_stack: Vec<(ecs::Entity, Spec, Option<ecs::Entity>)>,
  /// Resuable temporary storage for the stack of entities that need to be
  /// deleted.
  delete_stack: Vec<ecs::Entity>,
}

pub fn setup(engine: &mut Engine) {
  engine.resources.entry().or_insert_with(BuildState::default);
}

pub fn build(res: &ecs::Resources) {
  let entities = ecs::entities::read(res);
  let message_queue = &mut res.fetch_mut();
  let mut state = res.fetch_mut::<BuildState>();
  let mut nodes = nodes::write(res);

  // Clear the sorted hierarchy which is about to change.
  nodes.hierarchy.sorted.clear();

  // Push each hierarchy root entity onto the stack in reverse order.
  for root in nodes.roots().rev() {
    state.build_stack.push(root);
  }

  // For each entity on the stack…
  while let Some(entity) = state.build_stack.pop() {
    // By now, all entities before this one in the hierarchy have been
    // visited, so add this entity to the sorted vec.
    nodes.hierarchy.sorted.push(entity);

    let mut was_rebuilt = false;

    if let Some(node) = nodes.get_mut(entity) {
      // Awake the element. Does nothing if already awake.
      node.element.awake(NodeContext {
        entity,
        resources: res,
        entities: &entities,
        parent: node.parent,
        message_queue,
        should_rebuild: &mut node.should_rebuild,
      });

      if node.should_rebuild {
        let spec = node.element.build(
          ChildSpecs::new(&node.spec_children.entities),
          NodeContext {
            resources: res,
            entities: &entities,
            entity,
            parent: node.parent,
            message_queue,
            should_rebuild: &mut node.should_rebuild,
          },
        );

        push_apply_children(
          &mut state,
          NodeContext {
            resources: res,
            entities: &entities,
            entity,
            parent: node.parent,
            message_queue,
            // Ignored because the element was just built.
            should_rebuild: &mut false,
          },
          spec,
          &mut node.real_children,
        );

        was_rebuilt = true;
      }

      // Push all children onto the build stack in reverse order so that
      // they are sorted into the hierarchy correctly.
      state
        .build_stack
        .extend(node.real_children.entities.iter().rev().cloned());
    }

    if was_rebuilt {
      apply(res, &mut state, &entities, &mut nodes, message_queue);
    }
  }

  delete(res, &mut state, &entities, &mut nodes, message_queue);
}

fn apply(
  res: &ecs::Resources,
  state: &mut BuildState,
  entities: &ecs::Entities,
  nodes: &mut WriteNodes,
  message_queue: &mut MessageQueue,
) {
  while let Some((entity, spec, parent)) = state.apply_stack.pop() {
    let mut prototype = spec.into_element_prototype();
    let child_specs = mem::replace(&mut prototype.child_specs, Vec::new());

    // Apply the prototype to the entity and get its element node.
    let node = match nodes.get_mut(entity) {
      Some(node) => {
        // If the node already exists, update its element instance with the
        // element from the prototype.
        let result = node.element.replace_element(
          prototype.element,
          NodeContext {
            resources: res,
            entities,
            entity,
            parent: node.parent,
            message_queue,
            should_rebuild: &mut node.should_rebuild,
          },
        );

        if let Err(element) = result {
          // The element instance in the node has a different type than the
          // element in the prototype, so it needs to be fully replaced.
          node.element.sleep(NodeContext {
            resources: res,
            entities,
            entity,
            parent: node.parent,
            message_queue,
            should_rebuild: &mut node.should_rebuild,
          });

          node.element = (prototype.new)(
            element,
            NodeContext {
              resources: res,
              entities,
              entity,
              parent: node.parent,
              message_queue,
              should_rebuild: &mut node.should_rebuild,
            },
          );
        }

        node
      }

      None => {
        // If the node does not exist, create a new node with a new element
        // instance based on the prototype.
        let element = (prototype.new)(
          prototype.element,
          NodeContext {
            resources: res,
            entities,
            entity,
            parent,
            message_queue,
            // Ignored because the node is new and will be built regardless.
            should_rebuild: &mut true,
          },
        );

        nodes.create_on_entity(entity, element, parent)
      }
    };

    push_apply_children(
      state,
      NodeContext {
        resources: res,
        entities,
        entity,
        parent: node.parent,
        message_queue,
        should_rebuild: &mut node.should_rebuild,
      },
      child_specs,
      &mut node.spec_children,
    );
  }
}

fn delete(
  res: &ecs::Resources,
  state: &mut BuildState,
  entities: &ecs::Entities,
  nodes: &mut WriteNodes,
  message_queue: &mut MessageQueue,
) {
  while let Some(entity) = state.delete_stack.pop() {
    if let Some(node) = nodes.get_mut(entity) {
      node.element.sleep(NodeContext {
        resources: res,
        entities,
        entity,
        parent: node.parent,
        message_queue,
        should_rebuild: &mut node.should_rebuild,
      });

      push_delete_children(state, &mut node.spec_children, ..);
      push_delete_children(state, &mut node.real_children, ..);
    }

    nodes.delete(entity);

    let _ = entities.delete(entity);
  }
}

fn push_apply_children<I>(
  state: &mut BuildState,
  mut ctx: NodeContext,
  spec: I,
  children: &mut ChildNodes,
) where
  I: IntoIterator<Item = Spec>,
  I::IntoIter: ExactSizeIterator,
{
  let specs = spec.into_iter();

  let current_len = children.entities.len();
  let new_len = specs.len();

  // Flag any extra children for deletion.
  if new_len < current_len && current_len > 0 {
    push_delete_children(state, children, new_len..);
  }

  for (i, child) in specs.enumerate() {
    if let Some(child_entity) = child.entity() {
      // The child is an entity so reference it directly.

      if i < children.entities.len() {
        let existing = children.entities[i];

        if !children.references.remove(&existing) {
          state.delete_stack.push(existing);
        }

        children.entities[i] = child_entity;
      } else {
        children.entities.push(child_entity);
      }

      // Flag this child as a reference entity so it isn't deleted or
      // overwritten on rebuild.
      children.references.insert(child_entity);
    } else {
      // The child is an element so add it to the apply stack.

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

      state
        .apply_stack
        .push((children.entities[i], child, Some(ctx.entity)));
    }
  }

  // Rebuild is only needed if the number of children changes.
  if current_len != new_len {
    ctx.rebuild();
  }
}

fn push_delete_children(
  state: &mut BuildState,
  children: &mut ChildNodes,
  range: impl RangeBounds<usize>,
) {
  for entity in children.entities.drain(range) {
    if !children.references.remove(&entity) {
      state.delete_stack.push(entity);
    }
  }
}
