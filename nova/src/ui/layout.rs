// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::ecs;
use crate::el;
use crate::engine::{self, Engine};
use crate::math::Size;
use crate::window::Window;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Layout {
  pub top: Dimension,
  pub right: Dimension,
  pub bottom: Dimension,
  pub left: Dimension,
  pub width: Dimension,
  pub height: Dimension,
}

impl Layout {
  const DEFAULT: Self = Self {
    top: Dimension::Fixed(0.0),
    right: Dimension::Fixed(0.0),
    bottom: Dimension::Fixed(0.0),
    left: Dimension::Fixed(0.0),
    width: Dimension::Auto,
    height: Dimension::Auto,
  };
}

impl Default for Layout {
  fn default() -> Self {
    Self::DEFAULT
  }
}

impl ecs::Component for Layout {
  type Storage = ecs::BTreeStorage<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
  Auto,
  Fixed(f32),
}

#[derive(Debug, Copy, Clone)]
pub struct ScreenRect {
  pub left: f32,
  pub top: f32,
  pub size: Size<f32>,
}

impl ecs::Component for ScreenRect {
  type Storage = ecs::BTreeStorage<Self>;
}

#[derive(Debug, Default)]
pub struct SolveLayout {}

impl<'a> ecs::System<'a> for SolveLayout {
  type SystemData = (
    ecs::ReadResource<'a, el::Hierarchy>,
    ecs::ReadComponents<'a, el::hierarchy::Node>,
    ecs::ReadResource<'a, Window>,
    ecs::ReadComponents<'a, Layout>,
    ecs::WriteComponents<'a, ScreenRect>,
  );

  fn run(&mut self, (hierarchy, nodes, window, layouts, mut screen_rects): Self::SystemData) {
    let mut stack = Vec::new();

    let window_rect = ScreenRect {
      left: 0.0,
      top: 0.0,
      size: window.size().into(),
    };

    for root in hierarchy.roots() {
      stack.push((root, window_rect));
    }

    while let Some((entity, parent_rect)) = stack.pop() {
      let layout = layouts.get(entity).unwrap_or(&Layout::DEFAULT);

      let (left, width) = solve_dimension(
        parent_rect.size.width,
        layout.left,
        layout.width,
        layout.right,
      );

      let (top, height) = solve_dimension(
        parent_rect.size.height,
        layout.top,
        layout.height,
        layout.bottom,
      );

      let rect = ScreenRect {
        left: parent_rect.left + left,
        top: parent_rect.top + top,
        size: Size::new(width, height),
      };

      screen_rects.insert(entity, rect).unwrap();

      if let Some(node) = nodes.get(entity) {
        for child in node.children() {
          stack.push((child, rect));
        }
      }
    }
  }
}

pub fn setup(engine: &mut Engine) {
  engine.on_event(engine::Event::TickEnding, SolveLayout::default());
}

fn solve_dimension(
  mut total: f32,
  to_start: Dimension,
  middle: Dimension,
  from_end: Dimension,
) -> (f32, f32) {
  let mut result = (0.0, 0.0);
  let mut autos = 3;

  if let Dimension::Fixed(to_start) = to_start {
    result.0 = to_start;

    total -= to_start;
    autos -= 1;
  }

  if let Dimension::Fixed(middle) = middle {
    result.1 = middle;

    total -= middle;
    autos -= 1;
  }

  if let Dimension::Fixed(from_end) = from_end {
    total -= from_end;
    autos -= 1;
  }

  if autos > 0 {
    let part = total / autos as f32;

    if let Dimension::Auto = to_start {
      result.0 = part;
    }

    if let Dimension::Auto = middle {
      result.1 = part.max(0.0);
    }
  }

  result
}
