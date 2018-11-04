// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Hierarchy, Layout};
use crate::graphics::{Canvas, Color};
use crate::prelude::*;
use std::sync::{Arc, Mutex};

/// Component that stores the style of a panel.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Style {
  /// Background color of the panel.
  pub color: Color,
  pub background: Background,
  /// Custom draw implementation used to draw panel content.
  pub custom_draw: Option<Arc<Mutex<dyn CustomDraw>>>,
}

pub enum Background {
  None,
  Solid,
}

impl Style {
  /// Sets a `CustomDraw` for drawing custom content.
  pub fn set_custom_draw(&mut self, custom_draw: impl CustomDraw) {
    self.custom_draw = Some(Arc::new(Mutex::new(custom_draw)));
  }
}

impl Default for Style {
  fn default() -> Self {
    Style {
      color: Color([1.0, 1.0, 1.0, 1.0]),
      background: Background::None,
      custom_draw: None,
    }
  }
}

/// Trait for types that implement custom drawing for panel content.
pub trait CustomDraw: Send + 'static {
  fn draw(&mut self, ctx: &mut engine::Context, canvas: &mut Canvas, rect: &Rect<f32>);
}

// Implements custom draw for functions with the correct arguments.
impl<T> CustomDraw for T
where
  T: Fn(&mut engine::Context, &mut Canvas, &Rect<f32>) + Send + 'static,
{
  fn draw(&mut self, ctx: &mut engine::Context, canvas: &mut Canvas, rect: &Rect<f32>) {
    self(ctx, canvas, rect);
  }
}

/// Draws a panel and its children to the canvas.
pub fn draw(ctx: &mut engine::Context, canvas: &mut Canvas, panel: Entity) {
  // Begin with a stack containing the given panel.
  let mut stack = Vec::new();

  stack.push(panel);

  // Repeatedly re-enter the `draw_stack` function with the stack as it
  // “yields” tuples describing custom draw functions.
  //
  // Because custom draws need mutable access to the engine context, it is not
  // possible to keep a reference to a component storage alive when performing
  // one. This re-entrant behaviour fetches component storages once for every
  // “stretch” of panels that do not custom render.
  while let Some(custom) = draw_stack(ctx, canvas, &mut stack) {
    let (custom, rect) = custom;

    custom
      .lock()
      .expect("could not lock CustomDraw")
      .draw(ctx, canvas, &rect);
  }

  /// Local function that draws the stack until it finds an entity with a custom
  /// draw implementation, at which point it “yields” by returning information
  /// on the custom draw.
  fn draw_stack(
    ctx: &mut engine::Context,
    canvas: &mut Canvas,
    stack: &mut Vec<Entity>,
  ) -> Option<(Arc<Mutex<CustomDraw>>, Rect<f32>)> {
    let hierarchy = engine::fetch_storage::<Hierarchy>(ctx);
    let layouts = engine::fetch_storage::<Layout>(ctx);
    let styles = engine::fetch_storage::<Style>(ctx);

    // Pop elements off of the stack until it is empty.
    while let Some(entity) = stack.pop() {
      // If this entity has a hierarchy, push all of its children onto the stack.
      if let Some(node) = hierarchy.get(entity) {
        for child in node.children() {
          stack.push(*child);
        }
      }

      if let Some(layout) = layouts.get(entity) {
        // If this entity has layout and style, draw it.
        if let Some(style) = styles.get(entity) {
          draw_panel(canvas, layout.absolute_rect(), style);

          if let Some(ref custom_draw) = style.custom_draw {
            return Some((custom_draw.clone(), layout.absolute_rect().clone()));
          }
        }
      }
    }

    None
  }
}

/// Draws a panel with the given `rect` and `style`.
pub fn draw_panel(canvas: &mut Canvas, rect: &Rect<f32>, style: &Style) {
  match style.background {
    Background::Solid if style.color.a() > 0.0 => {
      let size = &rect.size;
      let pos = &rect.pos;

      let transform = Matrix4::new_translation(&Vector3::new(0.5, 0.5, 0.0))
        .append_nonuniform_scaling(&Vector3::new(size.x, size.y, 1.0))
        .append_translation(&Vector3::new(pos.x, pos.y, 0.0));

      canvas.set_tint(style.color);
      canvas.set_transform(transform);
      canvas.draw_quad();
    }

    _ => {}
  }
}
