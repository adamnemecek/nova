// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

#[derive(Component, Default)]
#[storage(BTreeStorage)]
pub struct Hierarchy {
  parent: Option<Entity>,
  children: Vec<Entity>,
}

/// Resource that stores the root panel entity.
pub struct Root {
  pub entity: Option<Entity>,
}

impl Hierarchy {
  pub fn parent(&self) -> &Option<Entity> {
    &self.parent
  }

  pub fn children(&self) -> &[Entity] {
    &self.children
  }
}

pub fn add_to_root(ctx: &mut engine::Context, child: Entity) {
  let root = engine::fetch_resource::<Root>(ctx).entity;

  set_parent(ctx, child, root);
}

pub fn set_parent(ctx: &mut engine::Context, child: Entity, parent: Option<Entity>) {
  let mut hierarchy = engine::fetch_storage_mut::<Hierarchy>(ctx);

  let old_parent = {
    let child_node = hierarchy
      .get_mut(child)
      .expect("entity does not have Hierarchy component");

    let old = child_node.parent;

    if parent == old {
      return;
    }

    child_node.parent = parent;
    old
  };

  if let Some(parent) = parent {
    let node = hierarchy
      .get_mut(parent)
      .expect("parent entity does not have Hierarchy component");

    node.children.push(child);
  } else if let Some(old_parent) = old_parent {
    if let Some(node) = hierarchy.get_mut(old_parent) {
      node.children.retain(|c| *c != child);
    }
  }
}
