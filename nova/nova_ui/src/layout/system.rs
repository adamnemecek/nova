// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::layout::{Constraints, HorizontalAlign, Layout, VerticalAlign};
use crate::nodes::ReadNodes;
use crate::screen::{Screen, ScreenRect};
use nova_core::components::{self, ReadComponents, WriteComponents};
use nova_core::engine::{Engine, EnginePhase};
use nova_core::entities::Entity;
use nova_core::math::{Rect, Size};
use nova_core::resources::ReadResource;
use nova_core::systems::derive::*;
use nova_core::systems::System;
use std::f32;

#[derive(Debug)]
struct LayoutElements;

#[derive(SystemData)]
struct InputData<'a> {
  nodes: ReadNodes<'a>,
  screen: ReadResource<'a, Screen>,
  layouts: ReadComponents<'a, Layout>,
}

#[derive(SystemData)]
struct OutputData<'a> {
  rects: WriteComponents<'a, ScreenRect>,
}

impl<'a> System<'a> for LayoutElements {
  type Data = (InputData<'a>, OutputData<'a>);

  fn run(&mut self, (input, mut output): Self::Data) {
    let screen_size = input.screen.size();

    let constraints = Constraints {
      min: Size::default(),
      max: screen_size,
    };

    for root in input.nodes.roots() {
      let size = calculate_size(&input, &mut output, root, constraints);

      let x = (screen_size.width - size.width) / 2.0;
      let y = (screen_size.height - size.height) / 2.0;

      offset_children(
        &input,
        &mut output,
        root,
        Rect {
          x1: x,
          y1: y,
          x2: x + size.width,
          y2: y + size.height,
        },
      );
    }
  }
}

pub fn setup(engine: &mut Engine) {
  components::register::<Layout>(&mut engine.resources);

  engine.schedule(EnginePhase::AfterUpdate, LayoutElements);
}

fn calculate_size(
  input: &InputData,
  output: &mut OutputData,
  entity: Entity,
  constraints: Constraints,
) -> Size<f32> {
  let layout = input.layouts.get(entity).cloned().unwrap_or_default();

  let size = match layout {
    Layout::Constrained(layout_constraints) => {
      let constraints = (layout_constraints * input.screen.dpi()).narrow_by(constraints);

      stack_children(input, output, entity, constraints)
    }

    Layout::Fill => {
      let constraints = Constraints {
        min: constraints.largest_finite_size(),
        max: constraints.max,
      };

      stack_children(input, output, entity, constraints)
    }

    Layout::AspectRatioFill(mut ratio) => {
      if ratio == 0.0 {
        for child in input.nodes.children_of(entity) {
          let child_layout = input.layouts.get(child);

          if let Some(Layout::AspectRatioFill(r)) = child_layout {
            ratio = ratio.max(*r);
          } else if let Some(Layout::Constrained(c)) = child_layout {
            ratio = ratio.max(c.largest_finite_size().ratio());
          }
        }
      }

      let constraints = if ratio == 0.0 {
        constraints
      } else if constraints.max.width.is_finite() {
        let height = (constraints.max.width / ratio)
          .max(constraints.min.height)
          .min(constraints.max.height);

        Size::new(height * ratio, height).into()
      } else if constraints.max.height.is_finite() {
        let width = (constraints.max.height * ratio)
          .max(constraints.min.width)
          .min(constraints.max.width);

        Size::new(width, width / ratio).into()
      } else {
        constraints
      };

      stack_children(input, output, entity, constraints)
    }

    Layout::Align(_, _) => {
      let size = stack_children(input, output, entity, constraints);

      let constraints = Constraints {
        min: constraints.largest_finite_size(),
        max: constraints.max,
      };

      constraints.constrain(size)
    }
  };

  output
    .rects
    .insert(
      entity,
      ScreenRect(Rect {
        x1: 0.0,
        y1: 0.0,
        x2: size.width,
        y2: size.height,
      }),
    )
    .unwrap();

  size
}

fn stack_children(
  input: &InputData,
  output: &mut OutputData,
  entity: Entity,
  constraints: Constraints,
) -> Size<f32> {
  let mut size = constraints.min;

  for child in input.nodes.children_of(entity) {
    let child_size = calculate_size(input, output, child, constraints);

    size.width = size.width.max(child_size.width).min(constraints.max.width);

    size.height = size
      .width
      .max(child_size.height)
      .min(constraints.max.height);
  }

  size
}

fn offset_children(input: &InputData, output: &mut OutputData, entity: Entity, rect: Rect<f32>) {
  let layout = input.layouts.get(entity);

  let (h_align, v_align) = match layout {
    Some(Layout::Align(h, v)) => (*h, *v),
    _ => (HorizontalAlign::Center, VerticalAlign::Center),
  };

  for child in input.nodes.children_of(entity) {
    let child_rect = output.rects.get_mut(child).unwrap();

    let x = match h_align {
      HorizontalAlign::Left => rect.x1,
      HorizontalAlign::Center => rect.x1 + (rect.width() - child_rect.width()) / 2.0,
      HorizontalAlign::Right => rect.x2 - child_rect.width(),
    };

    let y = match v_align {
      VerticalAlign::Top => rect.y1,
      VerticalAlign::Center => rect.y1 + (rect.height() - child_rect.height()) / 2.0,
      VerticalAlign::Bottom => rect.y2 - child_rect.height(),
    };

    child_rect.x1 += x;
    child_rect.y1 += y;
    child_rect.x2 += x;
    child_rect.y2 += y;

    let final_rect = child_rect.0;

    offset_children(input, output, child, final_rect);
  }
}
