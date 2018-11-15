use super::Dimension;
use super::Panel;
use nova::ecs::*;
use nova::math::{Point2, Rect, Size};

pub struct LayoutUpdater {
  stack: Vec<(Entity, Rect<f32>)>,
  size: Size<f32>,
}

impl LayoutUpdater {
  pub fn new(size: Size<f32>) -> Self {
    LayoutUpdater {
      stack: Vec::new(),
      size,
    }
  }
}

impl<'a> System<'a> for LayoutUpdater {
  type Data = (ReadEntities<'a>, WriteStorage<'a, Panel>);

  fn run(&mut self, (entities, mut panels): Self::Data) {
    // Begin with a stack of all root panels using the entire viewport as the
    // parent rect.
    self.stack.clear();

    let viewport = Rect {
      pos: Point2::origin(),
      size: self.size,
    };

    self.stack.extend(
      (&entities, &panels)
        .join()
        .filter(|(_, p)| p.parent.is_none())
        .map(|(e, _)| (e, viewport)),
    );

    // Pop an entity and parent rect off of the stack until it is empty.
    while let Some((entity, parent_rect)) = self.stack.pop() {
      let panel = panels.get_mut(entity).unwrap();

      // Compute its x and y dimensions relative to the parent rect.
      let x = solve_dimensions(
        parent_rect.size.height(),
        &panel.left,
        &panel.width,
        &panel.right,
      );

      let y = solve_dimensions(
        parent_rect.size.width(),
        &panel.top,
        &panel.height,
        &panel.bottom,
      );

      // Set its local rect with those dimensions.
      panel.rect = Rect::new(x.0, y.0, x.1, y.1);

      // Offset the local rect by the parent's position to get the absolute
      // rect.
      panel.absolute_rect = panel.rect + parent_rect.pos.coords;

      // Add all children of this panel to the stack with this rect as the
      // parent rect.
      for child in &panel.children {
        self.stack.push((*child, panel.absolute_rect));
      }
    }
  }
}

/// Computes the position and size given a start, size, and end dimension.
///
/// For example, passing in the left, width, and right dimensions would return
/// the appopriate x-coordinate and width for those dimensions.
fn solve_dimensions(
  full: f32,
  start_dim: &Dimension,
  size_dim: &Dimension,
  end_dim: &Dimension,
) -> (f32, f32) {
  // Amount of remaining space.
  let mut remaining = full;

  // Values for start and size if they have been found.
  let mut start = None;
  let mut size = None;

  // Number of unknown values (start, size, and end).
  let mut unknowns = 3;

  // 1. First subtract all fixed dimensions from remaining space.
  if let Dimension::Fixed(value) = start_dim {
    remaining -= value;
    start = Some(*value);
    unknowns -= 1;
  }

  if let Dimension::Fixed(value) = size_dim {
    remaining -= value;
    size = Some(*value);
    unknowns -= 1;
  }

  if let Dimension::Fixed(value) = end_dim {
    remaining -= value;
    unknowns -= 1;
  }

  // 2. Then, if the size is `Auto`, it consumes all remaining space.
  if let Dimension::Auto = size_dim {
    size = Some(remaining);
    remaining = 0.0;
    unknowns -= 1;
  }

  // 3. Lastly, if the start dimension is `Auto`, it consumes an equal share of
  //    the remaining space (half if the end dimension is `Auto` or all of it
  //    otherwise).
  if let Dimension::Auto = start_dim {
    start = Some(remaining / unknowns as f32);
  }

  // Return start and size values which have been calculated by now.
  (start.unwrap(), size.unwrap())
}
