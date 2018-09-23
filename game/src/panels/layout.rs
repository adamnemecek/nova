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
  root_rect: Rect,
}

#[derive(Clone, Copy)]
pub struct Rect {
  pub offset: Vector2<f32>,
  pub size: Vector2<f32>,
}

impl Default for Rect {
  fn default() -> Rect {
    Rect {
      offset: Vector2::zeros(),
      size: Vector2::zeros(),
    }
  }
}

impl Layout {
  pub fn root_rect(&self) -> &Rect {
    &self.root_rect
  }
}

impl Default for Layout {
  fn default() -> Self {
    Layout {
      offset: Vector2::new(32.0, 32.0),
      size: Vector2::new(100.0, 100.0),
      root_rect: Rect::default(),
    }
  }
}

pub struct LayoutSolver {
  pub root: Entity,
  stack: Vec<(Entity, Rect)>,
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
    self.stack.push((self.root, Rect::default()));

    while let Some((entity, parent_rect)) = self.stack.pop() {
      if let Some(layout) = layouts.get_mut(entity) {
        layout.root_rect = Rect {
          offset: parent_rect.offset + layout.offset,
          size: layout.size,
        };

        if let Some(node) = hierarchy.get(entity) {
          for child in node.children() {
            self.stack.push((*child, layout.root_rect));
          }
        }
      }
    }
  }
}
