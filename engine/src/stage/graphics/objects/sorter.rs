// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::DrawState;
use crate::prelude::*;
use crate::stage::objects::Object;
use crate::stage::Position;

/// Sorts objects on the stage by y-position and stores them in
/// the `State` resource.
pub struct Sorter;

impl<'a> System<'a> for Sorter {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Object>,
    ReadStorage<'a, Position>,
    WriteResource<'a, DrawState>,
  );

  fn run(&mut self, (entities, objects, positions, mut state): Self::SystemData) {
    // Clear the list of entities from last frame.
    //
    // TODO: Instead, use flagged storage to keep track of what is rendered.
    state.entities.clear();

    // Fill the buffer with all object entities.
    for (entity, _) in (&*entities, &objects).join() {
      state.entities.push(entity);
    }

    // Sort by y-position.
    state.entities.sort_by(|a, b| {
      if let Some(a) = positions.get(*a) {
        if let Some(b) = positions.get(*b) {
          if let Some(ord) = a.point.y.partial_cmp(&b.point.y) {
            return ord;
          }
        }
      }

      return a.cmp(b);
    });
  }
}
