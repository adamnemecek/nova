// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `rendering` module deals with drawing the stage onto the screen.
//!
//! The `Renderer` struct can be used in a game loop to draw the stage each
//! frame. The renderer will attempt to draw all entities with the `IsRendered`
//! component, sorting the draw calls by entity position.
//!
//! The stage is drawn relative to the target of the `Camera` resource.

use super::*;

mod camera;
mod renderer;

pub use self::camera::*;
pub use self::renderer::*;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct IsRendered;

/// Sets up stage components, resources, and systems.
pub fn setup<'a, 'b>(core: &mut Core, _dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<IsRendered>();

  core.world.add_resource(Camera::default());
}
