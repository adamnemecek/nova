// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::mount;
use super::{ChildNodes, Mount, Node, RebuildRequired, ShouldRebuild};
use crate::ecs;
use crate::engine;
use std::ops::RangeBounds;

#[derive(Debug, Default)]
pub struct Hierarchy {
  pub roots: Vec<ecs::Entity>,
  pub sorted: Vec<ecs::Entity>,
}

#[derive(Debug, Default)]
pub struct BuildHierarchy {
  // Resuable temporary storage for the stack of entities that need to be built.
  build_stack: Vec<ecs::Entity>,
  // Resuable temporary storage for the stack of entities that need new nodes
  // applied.
  apply_stack: Vec<(ecs::Entity, Node)>,
  // Resuable temporary storage for the stack of entities that need to be
  // deleted.
  delete_stack: Vec<ecs::Entity>,
}

impl<'a> ecs::System<'a> for BuildHierarchy {
  type SystemData = (
    ecs::ReadResource<'a, ecs::Entities>,
    ecs::WriteResource<'a, Hierarchy>,
    ecs::WriteComponents<'a, Mount>,
    ecs::WriteComponents<'a, RebuildRequired>,
  );

  fn setup(&mut self, res: &mut engine::Resources) {
    use ecs::SystemData;

    Self::SystemData::setup(res);

    res.entry().or_insert_with(Hierarchy::default);
  }

  fn run(&mut self, (entities, mut hierarchy, mut mounts, mut rebuild_required): Self::SystemData) {
    // Clear the sorted hierarchy which is about to change.
    hierarchy.sorted.clear();

    // Push each hierarchy root entity onto the stack in reverse order.
    for root in hierarchy.roots.iter().rev() {
      self.build_stack.push(*root);
    }

    // For each entity on the stack…
    while let Some(entity) = self.build_stack.pop() {
      // By now, all entities before this one in the hierarchy have been
      // visited, so add this entity to the sorted vec.
      hierarchy.sorted.push(entity);

      let mut was_rebuilt = false;

      if let Some(mount) = mounts.get_mut(entity) {
        // Awake the element. Does nothing if already awake.
        mount.instance.awake();

        // Rebuild the element if needed.
        if rebuild_required.contains(entity) {
          let node = mount.instance.build(ChildNodes {
            entities: mount.node_children.entities.iter(),
          });

          self.apply_node_to_children(node, &mut mount.real_children, &entities);

          was_rebuilt = true;
          rebuild_required.remove(entity);
        }

        // Push all children onto the build stack in reverse order so that
        // they are sorted into the hierarchy correctly.
        self
          .build_stack
          .extend(mount.real_children.entities.iter().rev().cloned());
      }

      if was_rebuilt {
        self.apply_node_changes(&entities, &mut mounts, &mut rebuild_required);
      }
    }

    self.delete_orphaned_entities(&entities, &mut mounts);
  }
}

impl BuildHierarchy {
  fn apply_node_changes(
    &mut self,
    entities: &ecs::Entities,
    mounts: &mut ecs::WriteComponents<Mount>,
    rebuild_required: &mut ecs::WriteComponents<RebuildRequired>,
  ) {
    while let Some((entity, node)) = self.apply_stack.pop() {
      let prototype = node.into_element_prototype();
      let mut should_rebuild = ShouldRebuild(true);

      // Apply the prototype to the entity and get its element mount.
      let mount = match mounts.get_mut(entity) {
        Some(mount) => {
          // If the mount already exists, update its props.
          match mount.instance.replace_element(prototype.element) {
            Ok(rebuild) => {
              should_rebuild = rebuild;
            }

            Err(element) => {
              // The mounted instance has a different type of element, so
              // replace it with a new instance based on the prototype.
              mount.instance.sleep();
              mount.instance = (prototype.new)(element);
            }
          };

          mount
        }

        None => {
          // If the mount doesn't exist, mount a new element instance based
          // on the prototype.
          mounts
            .insert(entity, Mount::new((prototype.new)(prototype.element)))
            .expect("Could not create element mount");

          mounts
            .get_mut(entity)
            .expect("Could not get newly created element mount")
        }
      };

      if *self.apply_node_to_children(prototype.children, &mut mount.node_children, &entities) {
        *should_rebuild = true;
      }

      if *should_rebuild {
        let _ = rebuild_required.insert(entity, RebuildRequired);
      }
    }
  }

  fn apply_node_to_children<I>(
    &mut self,
    node: I,
    children: &mut mount::Children,
    entities: &ecs::Entities,
  ) -> ShouldRebuild
  where
    I: IntoIterator<Item = Node>,
    I::IntoIter: ExactSizeIterator,
  {
    let nodes = node.into_iter();

    let current_len = children.entities.len();
    let new_len = nodes.len();

    // Flag any extra children for deletion.
    if new_len < current_len && current_len > 0 {
      self.delete_children(children, new_len..);
    }

    for (i, child) in nodes.enumerate() {
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
            children.entities[i] = entities.create();
          }
        } else {
          children.entities.push(entities.create());
        }

        self.apply_stack.push((children.entities[i], child));
      }
    }

    // Rebuild is only needed if the number of children changes.
    ShouldRebuild(current_len != new_len)
  }

  fn delete_orphaned_entities(
    &mut self,
    entities: &ecs::Entities,
    mounts: &mut ecs::WriteComponents<Mount>,
  ) {
    while let Some(entity) = self.delete_stack.pop() {
      if let Some(mount) = mounts.get_mut(entity) {
        mount.instance.sleep();

        self.delete_children(&mut mount.node_children, ..);
        self.delete_children(&mut mount.real_children, ..);
      }

      mounts.remove(entity);

      let _ = entities.delete(entity);
    }
  }

  fn delete_children(&mut self, children: &mut mount::Children, range: impl RangeBounds<usize>) {
    for entity in children.entities.drain(range) {
      if !children.references.remove(&entity) {
        self.delete_stack.push(entity);
      }
    }
  }
}
