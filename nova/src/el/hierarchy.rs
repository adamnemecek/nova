// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{ChildNodes, Mount, Node, RebuildRequired, ShouldRebuild};
use crate::ecs;
use crate::engine;

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

      if let Some(mut mount) = mounts.get_mut(entity) {
        // Rebuild the element if needed.
        if rebuild_required.contains(entity) {
          self.build_mounted_element(&mut mount, &entities);

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
  fn build_mounted_element(&mut self, mount: &mut Mount, entities: &ecs::Entities) {
    // Build the element and get the resulting children.
    let children = mount.instance.build(ChildNodes {
      entities: mount.node_children.entities.iter(),
    });

    // Flag any extra children for deletion.
    let current_len = mount.real_children.entities.len();
    let new_len = children.len();

    if new_len < current_len {
      for entity in mount.real_children.entities.drain(current_len - 1..) {
        if !mount.real_children.references.remove(&entity) {
          self.delete_stack.push(entity);
        }
      }
    }

    // Ensure enough child entities exist and push each one onto the apply
    // stack to change its node content.
    for (i, child) in children.into_iter().enumerate() {
      if let Some(child_entity) = child.entity() {
        // Flag this child as a reference entity so it isn't deleted or
        // overwritten on rebuild.
        mount.real_children.references.insert(child_entity);

        // If the node is a child entity, link to it directly.
        if i >= mount.real_children.entities.len() {
          mount.real_children.entities.push(child_entity);
        } else {
          let existing = mount.real_children.entities[i];

          if !mount.real_children.references.remove(&existing) {
            self.delete_stack.push(existing);
          }

          mount.real_children.entities[i] = child_entity;
        }
      } else {
        if i >= mount.real_children.entities.len() {
          mount.real_children.entities.push(entities.create());
        } else {
          let existing = mount.real_children.entities[i];

          if !mount.real_children.references.remove(&existing) {
            self.delete_stack.push(existing);

            mount.real_children.entities[i] = entities.create();
          }
        }

        self
          .apply_stack
          .push((mount.real_children.entities[i], child));
      }
    }
  }

  fn apply_node_changes(
    &mut self,
    entities: &ecs::Entities,
    mounts: &mut ecs::WriteComponents<Mount>,
    rebuild_required: &mut ecs::WriteComponents<RebuildRequired>,
  ) {
    while let Some((entity, node)) = self.apply_stack.pop() {
      let prototype = node.into_element_prototype();

      // Apply the prototype to the entity and get its element mount.
      let mut should_rebuild = ShouldRebuild::Yes;

      let mount = match mounts.get_mut(entity) {
        Some(mount) => {
          // If the mount already exists, update its props.
          match mount.instance.set_props(prototype.props) {
            Ok(rebuild) => {
              should_rebuild = rebuild;
            }

            Err(props) => {
              // The mounted element is a different type of element, so
              // replace it with a new instance based on the prototype.
              mount.instance = (prototype.new)(props);
            }
          };

          mount
        }

        None => {
          // If the mount doesn't exist, mount a new element instance based
          // on the prototype.
          mounts
            .insert(entity, Mount::new((prototype.new)(prototype.props)))
            .expect("Could not create element mount");

          mounts
            .get_mut(entity)
            .expect("Could not get newly created element mount")
        }
      };

      // If the number of node children has changed, then this element needs
      // to be rebuilt.
      let current_len = mount.node_children.entities.len();
      let new_len = prototype.children.len();

      if current_len != new_len {
        should_rebuild = ShouldRebuild::Yes;
      }

      // Flag any extra children for deletion.
      if new_len < current_len && current_len > 0 {
        for entity in mount.node_children.entities.drain(current_len - 1..) {
          if !mount.node_children.references.remove(&entity) {
            self.delete_stack.push(entity);
          }
        }
      }

      // Ensure enough child entities exist and flag each one for
      // application of the new node content.
      for (i, child) in prototype.children.into_iter().enumerate() {
        if let Some(child_entity) = child.entity() {
          // Flag this child as a reference entity so it isn't deleted or
          // overwritten on rebuild.
          mount.node_children.references.insert(child_entity);

          // If the node is a child entity, link to it directly.
          if i >= mount.node_children.entities.len() {
            mount.node_children.entities.push(child_entity);
          } else {
            let existing = mount.node_children.entities[i];

            if !mount.node_children.references.remove(&existing) {
              self.delete_stack.push(existing);
            }

            mount.node_children.entities[i] = child_entity;
          }
        } else {
          if i >= mount.node_children.entities.len() {
            mount.node_children.entities.push(entities.create());
          } else {
            let existing = mount.node_children.entities[i];

            if !mount.node_children.references.remove(&existing) {
              self.delete_stack.push(existing);

              mount.node_children.entities[i] = entities.create();
            }
          }

          self
            .apply_stack
            .push((mount.node_children.entities[i], child));
        }
      }

      // If the element should be rebuilt, flag it for rebuilding.
      if let ShouldRebuild::Yes = should_rebuild {
        let _ = rebuild_required.insert(entity, RebuildRequired);
      }
    }
  }

  fn delete_orphaned_entities(
    &mut self,
    entities: &ecs::Entities,
    mounts: &mut ecs::WriteComponents<Mount>,
  ) {
    while let Some(entity) = self.delete_stack.pop() {
      if let Some(mount) = mounts.get_mut(entity) {
        for entity in &mount.node_children.entities {
          if !mount.node_children.references.contains(entity) {
            self.delete_stack.push(*entity);
          }
        }

        for entity in &mount.real_children.entities {
          if !mount.real_children.references.contains(entity) {
            self.delete_stack.push(*entity);
          }
        }
      }

      mounts.remove(entity);

      let _ = entities.delete(entity); // Err is if the entity is already deleted. ¯\_(ツ)_/¯
    }
  }
}
