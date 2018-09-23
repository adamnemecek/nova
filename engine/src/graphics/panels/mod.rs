// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Canvas;
use crate::prelude::*;

mod drawing;
mod hierarchy;
mod layout;
mod rect;

pub use self::drawing::*;
pub use self::hierarchy::*;
pub use self::layout::*;
pub use self::rect::*;

/// Initializes panels for the given engine context.
pub fn init(ctx: &mut engine::Context, canvas: Canvas) {
  engine::add_storage::<Hierarchy>(ctx);
  engine::add_storage::<Layout>(ctx);
  engine::add_storage::<Style>(ctx);

  let root = create_panel(ctx);

  engine::add_resource(ctx, Root { entity: Some(root) });

  engine::init::add_system_late(
    ctx,
    LayoutSolver::new(root),
    "graphics::panels::LayoutSolver",
    &[],
  );

  engine::init::add_process(ctx, RootDrawer { canvas });
}

/// Creates a new panel entity in the given engine context.
pub fn create_panel(ctx: &mut engine::Context) -> Entity {
  engine::build_entity(ctx)
    .with(Layout::default())
    .with(Hierarchy::default())
    .with(Style::default())
    .build()
}
