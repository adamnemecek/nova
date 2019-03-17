// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::collections::FnvHashSet;
use crate::ecs::resources::{ResourceId, Resources};
use crate::ThreadPool;
use specs::shred::RunWithPool;
use std::fmt;

/// Generic trait for an operation that can be run by a `Scheduler`.
pub trait Runnable: fmt::Debug {
  fn run(&mut self, resources: &Resources, thread_pool: &ThreadPool);

  /// Adds to the given set the resources this runnable needs to read.
  fn reads(&self, set: &mut FnvHashSet<ResourceId>);

  /// Adds to the given set the resources this runnable needs to write.
  fn writes(&self, set: &mut FnvHashSet<ResourceId>);
}

// Blanket implementation for `RunWithPool`, shred's equivalent trait.
impl<T> Runnable for T
where
  T: for<'a> RunWithPool<'a> + fmt::Debug,
{
  fn run(&mut self, resources: &Resources, thread_pool: &ThreadPool) {
    RunWithPool::run(self, resources, thread_pool);
  }

  fn reads(&self, set: &mut FnvHashSet<ResourceId>) {
    let mut buffer = Vec::new();

    RunWithPool::reads(self, &mut buffer);

    set.extend(buffer.into_iter());
  }

  fn writes(&self, set: &mut FnvHashSet<ResourceId>) {
    let mut buffer = Vec::new();

    RunWithPool::writes(self, &mut buffer);

    set.extend(buffer.into_iter());
  }
}
