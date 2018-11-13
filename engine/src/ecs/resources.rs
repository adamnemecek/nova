// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::shred::Resource;
pub use specs::shred::{Fetch as FetchResource, FetchMut as FetchResourceMut};
pub use specs::{ReadExpect as ReadResource, WriteExpect as WriteResource};

use super::Context;

pub fn add_resource(ctx: &mut Context, resource: impl Resource) {
  ctx.world.res.insert(resource);
}

pub fn get_resource_mut<T: Resource>(ctx: &mut Context) -> &mut T {
  ctx.world.res.get_mut().unwrap()
}
