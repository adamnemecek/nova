// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::Hierarchy;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Layout {
  pub top: LayoutDimension,
  pub left: LayoutDimension,
  pub width: LayoutDimension,
  pub height: LayoutDimension,
  rect: Rect,
  absolute_rect: Rect,
}

pub enum LayoutDimension {
  Auto,
  Fixed(f32),
}

impl Layout {
  pub fn absolute_rect(&self) -> &Rect {
    &self.absolute_rect
  }
}

impl Default for Layout {
  fn default() -> Self {
    Layout {
      top: LayoutDimension::Auto,
      left: LayoutDimension::Auto,
      width: LayoutDimension::Auto,
      height: LayoutDimension::Auto,
      rect: Rect::default(),
      absolute_rect: Rect::default(),
    }
  }
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
  type SystemData = (
    ReadResource<'a, engine::Window>,
    ReadStorage<'a, Hierarchy>,
    WriteStorage<'a, Layout>,
  );

  fn run(&mut self, (window, hierarchy, mut layouts): Self::SystemData) {
    self.stack.clear();

    self.stack.push((
      self.root,
      Rect {
        offset: Vector2::zeros(),
        size: window.size(),
      },
    ));

    while let Some((entity, parent_rect)) = self.stack.pop() {
      if let Some(layout) = layouts.get_mut(entity) {
        match layout.left {
          LayoutDimension::Fixed(left) => {
            layout.rect.offset.x = left;

            match layout.width {
              LayoutDimension::Fixed(width) => {
                layout.rect.size.x = width;
              }

              LayoutDimension::Auto => {
                layout.rect.size.x = parent_rect.size.x - left;
              }
            }
          }

          LayoutDimension::Auto => match layout.width {
            LayoutDimension::Fixed(width) => {
              layout.rect.offset.x = parent_rect.size.x - width;
              layout.rect.size.x = width;
            }

            LayoutDimension::Auto => {
              layout.rect.offset.x = 0.0;
              layout.rect.size.x = parent_rect.size.x;
            }
          },
        }

        match layout.top {
          LayoutDimension::Fixed(top) => {
            layout.rect.offset.y = top;

            match layout.height {
              LayoutDimension::Fixed(height) => {
                layout.rect.size.y = height;
              }

              LayoutDimension::Auto => {
                layout.rect.size.y = parent_rect.size.y - top;
              }
            }
          }

          LayoutDimension::Auto => match layout.height {
            LayoutDimension::Fixed(height) => {
              layout.rect.offset.y = parent_rect.size.y - height;
              layout.rect.size.y = height;
            }

            LayoutDimension::Auto => {
              layout.rect.offset.y = 0.0;
              layout.rect.size.y = parent_rect.size.y;
            }
          },
        }

        layout.absolute_rect = Rect {
          offset: parent_rect.offset + layout.rect.offset,
          size: layout.rect.size,
        };

        if let Some(node) = hierarchy.get(entity) {
          for child in node.children() {
            self.stack.push((*child, layout.absolute_rect));
          }
        }
      }
    }
  }
}
