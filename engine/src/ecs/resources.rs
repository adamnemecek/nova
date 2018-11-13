// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::shred::Resource;
pub use specs::shred::{Fetch as FetchResource, FetchMut as FetchResourceMut};
pub use specs::{ReadExpect as ReadResource, WriteExpect as WriteResource};

use super::Context;

/// Adds a resource to the ECS context. If the resource already existed, the old
/// value is overwritten.
pub fn put_resource(ctx: &mut Context, resource: impl Resource) {
  ctx.world.res.insert(resource);
}

/// Gets a mutable reference to a resource in an ECS context. If the resource
/// does not exist, this function will panic.
///
/// This is faster than fetching the resource but requires a mutable reference
/// to the context.
pub fn get_resource_mut<T: Resource>(ctx: &mut Context) -> &mut T {
  ctx
    .world
    .res
    .get_mut()
    .expect("The specified resource does not exist.")
}
