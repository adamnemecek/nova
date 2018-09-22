// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::Hierarchy;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Layout {
  pub offset: Vector2<f32>,
  pub size: Vector2<f32>,
  parent_offset: Vector2<f32>,
}

impl Layout {
  pub fn root_offset(&self) -> Vector2<f32> {
    self.parent_offset + self.offset
  }
}

impl Default for Layout {
  fn default() -> Self {
    Layout {
      offset: Vector2::new(32.0, 32.0),
      parent_offset: Vector2::zeros(),
      size: Vector2::new(100.0, 100.0),
    }
  }
}

pub struct LayoutSolver {
  pub root: Entity,
  stack: Vec<(Entity, Vector2<f32>)>,
}

impl LayoutSolver {
  pub fn new(root: Entity) -> Self {
    LayoutSolver {
      root,
      stack: Vec::new(),
    }
  }
}

impl<'a> System<'a> for LayoutSolver {
  type SystemData = (ReadStorage<'a, Hierarchy>, WriteStorage<'a, Layout>);

  fn run(&mut self, (hierarchy, mut layouts): Self::SystemData) {
    self.stack.clear();
    self.stack.push((self.root, Vector2::zeros()));

    while let Some((entity, offset)) = self.stack.pop() {
      if let Some(layout) = layouts.get_mut(entity) {
        layout.parent_offset = offset;

        if let Some(node) = hierarchy.get(entity) {
          for child in node.children() {
            self.stack.push((*child, layout.offset));
          }
        }
      }
    }
  }
}
