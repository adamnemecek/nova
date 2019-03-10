// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Dimension, Layout, ScreenRect};
use crate::Screen;
use nova_core::ecs;
use nova_core::el;
use nova_core::math::{Point2, Rect, Size};

#[derive(Debug)]
pub struct SolveLayout;

impl<'a> ecs::System<'a> for SolveLayout {
  type SystemData = (
    el::hierarchy::ReadHierarchyNodes<'a>,
    ecs::ReadResource<'a, Screen>,
    ecs::ReadComponents<'a, Layout>,
    ecs::WriteComponents<'a, ScreenRect>,
  );

  fn run(&mut self, (hierarchy, screen, layouts, mut screen_rects): Self::SystemData) {
    let mut stack = Vec::new();

    let screen_rect = ScreenRect(Rect::new(Point2::origin(), screen.size()));

    for root in hierarchy.roots() {
      stack.push((root, screen_rect));
    }

    let dpi = screen.dpi();

    while let Some((entity, parent_rect)) = stack.pop() {
      let parent_size = parent_rect.size();
      let layout = layouts.get(entity).unwrap_or(&Layout::DEFAULT);

      let (left, width) = solve_dimension(
        parent_size.width / dpi,
        layout.left,
        layout.width,
        layout.right,
      );

      let (top, height) = solve_dimension(
        parent_size.height / dpi,
        layout.top,
        layout.height,
        layout.bottom,
      );

      let rect = ScreenRect(Rect::new(
        Point2::new(parent_rect.x1 + left * dpi, parent_rect.y1 + top * dpi),
        Size::new(width * dpi, height * dpi),
      ));

      screen_rects.insert(entity, rect).unwrap();

      for child in hierarchy.get_children_of(entity) {
        stack.push((child, rect));
      }
    }
  }
}

fn solve_dimension(
  total: f32,
  to_start: Dimension,
  middle: Dimension,
  from_end: Dimension,
) -> (f32, f32) {
  let mut result = (0.0, 0.0);

  let mut remaining = total;
  let mut autos = 3;

  if let Some(to_start) = to_start.into_scalar(total) {
    result.0 = to_start;

    remaining -= to_start;
    autos -= 1;
  }

  if let Some(middle) = middle.into_scalar(total) {
    result.1 = middle;

    remaining -= middle;
    autos -= 1;
  }

  if let Some(from_end) = from_end.into_scalar(total) {
    remaining -= from_end;
    autos -= 1;
  }

  if autos > 0 {
    let part = remaining / autos as f32;

    if let Dimension::Auto = to_start {
      result.0 = part;
    }

    if let Dimension::Auto = middle {
      result.1 = part.max(0.0);
    }
  }

  result
}
