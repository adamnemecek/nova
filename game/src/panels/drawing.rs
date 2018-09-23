// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::{Hierarchy, Layout};
use graphics::{Canvas, Color, DrawLayer};

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Style {
  pub color: Color,
  pub background: Option<Arc<graphics::Image>>,
}

impl Default for Style {
  fn default() -> Self {
    Style {
      color: Color::new(1.0, 1.0, 1.0, 1.0),
      background: None,
    }
  }
}

pub struct PanelLayer {
  pub root: Entity,
}

#[derive(Default)]
pub struct PanelLayerState {
  stack: Vec<Entity>,
}

impl DrawLayer for PanelLayer {
  fn draw(&self, ctx: &mut engine::Context, canvas: &mut Canvas) {
    let mut state = engine::fetch_resource_mut::<PanelLayerState>(ctx);

    let hierarchy = engine::fetch_storage::<Hierarchy>(ctx);
    let layouts = engine::fetch_storage::<Layout>(ctx);
    let styles = engine::fetch_storage::<Style>(ctx);

    state.stack.clear();
    state.stack.push(self.root);

    while let Some(entity) = state.stack.pop() {
      if let Some(node) = hierarchy.get(entity) {
        for child in node.children() {
          state.stack.push(*child);
        }
      }

      if let Some(layout) = layouts.get(entity) {
        if let Some(style) = styles.get(entity) {
          draw_panel(canvas, layout, style);
        }
      }
    }
  }
}

fn draw_panel(canvas: &mut Canvas, layout: &Layout, style: &Style) {
  if let Some(ref background) = style.background {
    let bg_size = background.size();

    let rect = layout.root_rect();

    canvas
      .draw(
        background,
        graphics::DrawParams::default()
          .color(style.color)
          .dest(Point2::from_coordinates(rect.offset))
          .scale(Vector2::new(
            rect.size.x / bg_size.x as f32,
            rect.size.y / bg_size.y as f32,
          )),
      )
      .expect("could not draw panel background");
  }
}
