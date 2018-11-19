// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::shred::Resource;
pub use specs::shred::{Fetch as FetchResource, FetchMut as FetchResourceMut};
pub use specs::{ReadExpect as ReadResource, WriteExpect as WriteResource};

use crate::Engine;

/// Adds a resource to the engine instance. If the resource already existed, the
/// old value is overwritten.
pub fn put_resource(engine: &mut Engine, resource: impl Resource) {
  engine.world.res.insert(resource);
}

/// Gets a mutable reference to a resource in an engine instance. If the
/// resource does not exist, this function will panic.
///
/// This is faster than fetching the resource but requires a mutable reference
/// to the engine.
pub fn get_resource_mut<T: Resource>(engine: &mut Engine) -> &mut T {
  engine
    .world
    .res
    .get_mut()
    .expect("The specified resource does not exist.")
}
