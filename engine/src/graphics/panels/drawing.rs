// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::{Hierarchy, Layout, Rect, Root};
use graphics::{Canvas, Color};

/// Component that stores the style of a panel.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Style {
  /// Multiplicative color of the panel.
  pub color: Color,
  /// Background image of the panel.
  pub background: Option<Arc<graphics::Image>>,
  /// Custom draw implementation used to draw panel content.
  pub custom_draw: Option<Arc<Mutex<dyn CustomDraw>>>,
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
      color: Color::new(1.0, 1.0, 1.0, 1.0),
      background: None,
      custom_draw: None,
    }
  }
}

/// Trait for types that implement custom drawing for panel content.
pub trait CustomDraw: Send + 'static {
  fn draw(&mut self, ctx: &mut engine::Context, canvas: &mut graphics::Canvas, rect: &Rect);
}

// Implements custom draw for functions with the correct arguments.
impl<T> CustomDraw for T
where
  T: Fn(&mut engine::Context, &mut graphics::Canvas, &Rect) + Send + 'static,
{
  fn draw(&mut self, ctx: &mut engine::Context, canvas: &mut graphics::Canvas, rect: &Rect) {
    self(ctx, canvas, rect);
  }
}

/// Engine process that draws the root panel to the canvas.
pub struct RootDrawer {
  pub canvas: graphics::Canvas,
}

impl engine::Process for RootDrawer {
  fn late_update(&mut self, ctx: &mut engine::Context) {
    // Resize canvas to match window size.
    {
      let window = engine::fetch_resource::<engine::Window>(ctx);

      if window.was_resized() {
        self.canvas.resize(window.size());
      }
    }

    let root = engine::fetch_resource::<Root>(ctx).entity;

    if let Some(root) = root {
      self.canvas.clear(Color::new(0.086, 0.086, 0.114, 1.0));

      draw(ctx, &mut self.canvas, root);

      self.canvas.present();
    }
  }
}

/// Draws the given `panel` and its children.
fn draw(ctx: &mut engine::Context, canvas: &mut Canvas, panel: Entity) {
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
  ) -> Option<(Arc<Mutex<CustomDraw>>, Rect)> {
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

      // If this entity has layout and style, draw it.
      if let Some(layout) = layouts.get(entity) {
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
pub fn draw_panel(canvas: &mut Canvas, rect: &Rect, style: &Style) {
  // If the panel has a background image, draw it covering the entire rect of
  // the panel.
  if let Some(ref background) = style.background {
    let bg_size = background.size();

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
