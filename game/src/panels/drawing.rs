// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::{Hierarchy, Layout};
use graphics::{Canvas, Color, DrawLayer};

/// Component that stores the style of a panel.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Style {
  /// Multiplicative color of the panel.
  pub color: Color,
  /// Background image of the panel.
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

/// Draw layer that draws a hierarchy of panels.
pub struct PanelLayer {
  /// Root panel entity to draw.
  pub root: Entity,
}

impl DrawLayer for PanelLayer {
  fn draw(&self, ctx: &mut engine::Context, canvas: &mut Canvas) {
    let hierarchy = engine::fetch_storage::<Hierarchy>(ctx);
    let layouts = engine::fetch_storage::<Layout>(ctx);
    let styles = engine::fetch_storage::<Style>(ctx);

    // Begin with a stack containing the root element.
    let mut stack = Vec::new();

    stack.push(self.root);

    // Pop elements off of the stack until it is empty.
    while let Some(entity) = stack.pop() {
      // If this entity has a hierarchy, push all of its children onto the stack.
      if let Some(node) = hierarchy.get(entity) {
        for child in node.children() {
          stack.push(*child);
        }
      }

      // If this entity has layout and style, draw it.
      if let Some(layout) = layouts.get(entity) {
        if let Some(style) = styles.get(entity) {
          draw_panel(canvas, layout, style);
        }
      }
    }
  }
}

/// Draws a panel with the given `layout` and `style`.
fn draw_panel(canvas: &mut Canvas, layout: &Layout, style: &Style) {
  // If the panel has a background image, draw it covering the entire rect of
  // the panel.
  if let Some(ref background) = style.background {
    let bg_size = background.size();
    let rect = layout.absolute_rect();

    canvas
      .draw(
        background,
        graphics::DrawParams::default()
          .color(style.color)
          .dest(rect.position)
          .scale(Vector2::new(
            rect.size.x / bg_size.x as f32,
            rect.size.y / bg_size.y as f32,
          )),
      )
      .expect("could not draw panel background");
  }
}
