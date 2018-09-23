// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// Component that stores a panel entity's parent and children.
#[derive(Default, Component)]
#[storage(BTreeStorage)]
pub struct Hierarchy {
  parent: Option<Entity>,
  children: Vec<Entity>,
}

impl Hierarchy {
  /// Gets the children of the entity if it has any.
  pub fn children(&self) -> &[Entity] {
    &self.children
  }
}

/// Resource that stores the global root panel entity.
#[derive(Default)]
pub struct Root {
  /// Global root panel entity or `None` if no panel is set as root.
  pub entity: Option<Entity>,
}

/// Adds an entity to the children of the root panel.
pub fn add_to_root(ctx: &mut engine::Context, child: Entity) {
  let root = engine::fetch_resource::<Root>(ctx).entity;

  set_parent(ctx, child, root);
}

/// Sets the parent of the given `child` entity to the given `parent`.
pub fn set_parent(ctx: &mut engine::Context, child: Entity, parent: Option<Entity>) {
  let mut hierarchy = engine::fetch_storage_mut::<Hierarchy>(ctx);

  // Get the current parent as a result of:
  let current_parent = {
    // Get the child's current parent from the hierarchy.
    let child_node = hierarchy
      .get_mut(child)
      .expect("entity does not have Hierarchy component");

    let current_parent = child_node.parent;

    // If it is the same as the new parent, return immediately.
    if parent == current_parent {
      return;
    }

    // Otherwise change the parent to the new parent.
    child_node.parent = parent;

    current_parent
  };

  // If the child already has a parent, remove it from that parent's children.
  if let Some(current_parent) = current_parent {
    if let Some(node) = hierarchy.get_mut(current_parent) {
      node.children.retain(|c| *c != child);
    }
  }

  // If the child was just assigned a parent (i.e. not `None`), add it to that
  // parent's children.
  if let Some(parent) = parent {
    let node = hierarchy
      .get_mut(parent)
      .expect("parent entity does not have Hierarchy component");

    node.children.push(child);
  }
}
