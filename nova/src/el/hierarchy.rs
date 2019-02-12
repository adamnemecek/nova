// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Mount, Node, RebuildRequired, ShouldRebuild};
use crate::ecs;
use crate::engine;

#[derive(Debug, Default)]
pub struct Hierarchy {
  pub roots: Vec<ecs::Entity>,
  pub sorted: Vec<ecs::Entity>,
}

#[derive(Debug, Default)]
pub struct BuildHierarchy {
  build_stack: Vec<ecs::Entity>,
  apply_stack: Vec<(ecs::Entity, Node)>,
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
      println!("Building {}…", entity.id());

      // By now, all entities before this one in the hierarchy have been
      // visited, so add this entity to the sorted vec.
      hierarchy.sorted.push(entity);

      // If this entity is a mounted element…
      if let Some(element) = mounts.get_mut(entity) {
        // Rebuild the element if needed.
        if rebuild_required.contains(entity) {
          rebuild_required.remove(entity);

          // Build the element and get the resulting children.
          let children = element.instance.build();

          // Flag any extra children for deletion.
          let current_len = element.real_children.len();
          let new_len = children.len();

          if new_len > current_len && current_len > 0 {
            let extras = element.real_children.drain(current_len - 1..new_len);

            self.delete_stack.extend(extras);
          }

          // Ensure enough child entities exist and push each one onto the apply
          // stack to change its node content.
          for (i, child) in children.into_iter().enumerate() {
            if i >= element.real_children.len() {
              element.real_children.push(entities.create());
            }

            self.apply_stack.push((element.real_children[i], child));
          }
        }

        // Push all children onto the build stack in reverse order so that
        // they are sorted into the hierarchy correctly.
        self
          .build_stack
          .extend(element.real_children.iter().rev().cloned());
      }

      // Apply changes to node descendents.
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
        let current_len = mount.node_children.len();
        let new_len = prototype.children.len();

        if current_len != new_len {
          should_rebuild = ShouldRebuild::Yes;
        }

        // Flag any extra children for deletion.
        if new_len > current_len && current_len > 0 {
          let extras = mount.node_children.drain(current_len - 1..new_len);

          self.delete_stack.extend(extras);
        }

        // Ensure enough child entities exist and flag each one for
        // application of the new node content.
        for (i, child) in prototype.children.into_iter().enumerate() {
          if i >= mount.node_children.len() {
            mount.node_children.push(entities.create());
          }

          self.apply_stack.push((mount.node_children[i], child));
        }

        // If the element should be rebuilt, flag it for rebuilding.
        if let ShouldRebuild::Yes = should_rebuild {
          let _ = rebuild_required.insert(entity, RebuildRequired);
        }
      }
    }

    // Finally, delete all orphaned entities.
    while let Some(entity) = self.delete_stack.pop() {
      if let Some(mount) = mounts.get_mut(entity) {
        self.delete_stack.extend(mount.node_children.drain(..));
        self.delete_stack.extend(mount.real_children.drain(..));
      }

      mounts.remove(entity);

      let _ = entities.delete(entity); // Err is if the entity is already deleted. ¯\_(ツ)_/¯
    }
  }
}
