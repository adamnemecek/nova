// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::world::Builder as BuildEntity;
pub use specs::world::EntitiesRes as EntitiesResource;
pub use specs::world::{Entity, EntityBuilder};

use crate::components;
use crate::resources::{ReadResource, Resources};
use crate::systems::SystemData;

pub type Entities<'a> = ReadResource<'a, EntitiesResource>;

pub fn setup(resources: &mut Resources) {
  resources.entry().or_insert_with(EntitiesResource::default);
}

pub fn borrow(resources: &Resources) -> Entities {
  SystemData::fetch(resources)
}

pub(crate) fn maintain(resources: &mut Resources, buffer: &mut Vec<Entity>) {
  EntitiesResource::merge_deleted(&mut resources.fetch_mut(), buffer);

  if !buffer.is_empty() {
    components::delete_all_for_entities(resources, buffer);
  }
}
