// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::shred::MetaTable as ResourceMetaTable;
pub use specs::shred::{ReadExpect as ReadResource, WriteExpect as WriteResource};
pub use specs::shred::{Resource, ResourceId, Resources};

use specs::shred::SystemData;

/// Borrows a resource immutably.
pub fn borrow<R: Resource>(res: &Resources) -> ReadResource<R> {
  SystemData::fetch(res)
}

/// Borrows a resource mutably.
///
/// A resource can only be borrowed mutably if no other borrows exist on any
/// thread.
pub fn borrow_mut<R: Resource>(res: &Resources) -> WriteResource<R> {
  SystemData::fetch(res)
}
