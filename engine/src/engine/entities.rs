// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Context;

pub use specs::Builder as EntityBuilder;
pub use specs::{Entities, Entity};

/// Creates a new entity builder that will build an entity in the engine
/// context.
pub fn build_entity<'a>(ctx: &'a mut Context) -> impl EntityBuilder + 'a {
  ctx.world.create_entity()
}
