// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Children, Hierarchy, ShouldRebuild, Spec};
use crate::ecs;
use crate::engine;
use std::ops::RangeBounds;

pub struct Context<'a> {
  pub hierarchy: &'a mut Hierarchy,
  pub resources: &'a engine::Resources,
  pub entities: &'a ecs::Entities,
  pub entity: ecs::Entity,
}

impl<'a> Context<'a> {
  pub(crate) fn push_apply_children<I>(&mut self, spec: I, children: &mut Children) -> ShouldRebuild
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
            self.hierarchy.delete_stack.push(existing);
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
            children.entities[i] = self.entities.create();
          }
        } else {
          children.entities.push(self.entities.create());
        }

        self
          .hierarchy
          .apply_stack
          .push((children.entities[i], child));
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
        self.hierarchy.delete_stack.push(entity);
      }
    }
  }
}
