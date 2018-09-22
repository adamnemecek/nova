// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub use specs::shred::{Fetch, FetchMut, Read as ReadResource, Resource, Write as WriteResource};

/// Adds a resource to the engine context.
pub fn add_resource(ctx: &mut Context, resource: impl Resource) {
  ctx.world.add_resource(resource)
}

/// Fetches a resource from the engine context.
pub fn fetch_resource<'a, T: Resource + Send + 'a>(ctx: &'a Context) -> Fetch<'a, T> {
  ctx.world.read_resource::<T>()
}

/// Mutably fetches a resource from the engine context.
pub fn fetch_resource_mut<'a, T: Resource + Send + 'a>(ctx: &'a Context) -> FetchMut<'a, T> {
  ctx.world.write_resource::<T>()
}

/// Checks whether the engine context has a resource of type `T`.
pub fn has_resource<T: Resource + Send>(ctx: &Context) -> bool {
  ctx.world.res.has_value::<T>()
}
