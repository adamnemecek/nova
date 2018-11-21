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

/// Fetches a reference to a resource in the engine instance.
///
/// # Panics
///
/// This function panics if the resource does not exist or is currently fetched
/// mutably.
pub fn fetch_resource<T: Resource>(engine: &Engine) -> FetchResource<T> {
  engine.world.res.fetch()
}

/// Gets a mutable reference to a resource in an engine instance. This is more
/// efficient than fetching a resource.
///
/// # Panics
///
/// This function panics if the resource does not exist.
pub fn get_resource_mut<T: Resource>(engine: &mut Engine) -> &mut T {
  engine
    .world
    .res
    .get_mut()
    .expect("The specified resource does not exist.")
}
