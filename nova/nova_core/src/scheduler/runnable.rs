// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::collections::FnvHashSet;
use crate::resources::{ResourceId, Resources};
use crate::scheduler::ThreadPool;
use crate::systems::{System, SystemData};
use std::fmt;

/// Generic trait for an operation that can be run by a `Scheduler`.
pub trait Runnable<'a>: fmt::Debug {
  fn run(&mut self, resources: &'a Resources, thread_pool: &ThreadPool);

  /// Adds to the given set the resources this runnable needs to read.
  fn reads(&self, set: &mut FnvHashSet<ResourceId>);

  /// Adds to the given set the resources this runnable needs to write.
  fn writes(&self, set: &mut FnvHashSet<ResourceId>);
}

// Blanket implementation for `RunWithPool`, shred's equivalent trait.
impl<'a, T> Runnable<'a> for T
where
  T: System<'a> + fmt::Debug,
{
  fn run(&mut self, resources: &'a Resources, _: &ThreadPool) {
    System::run(self, SystemData::fetch(resources));
  }

  fn reads(&self, set: &mut FnvHashSet<ResourceId>) {
    let buffer = T::Data::reads();

    set.extend(buffer.into_iter());
  }

  fn writes(&self, set: &mut FnvHashSet<ResourceId>) {
    let buffer = T::Data::writes();

    set.extend(buffer.into_iter());
  }
}
