// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::collections::FnvHashSet;
use crate::ecs::resources::{ResourceId, Resources};
use rayon::ThreadPool;
use specs::shred::RunWithPool;
use std::fmt;

type ParRunnable = Box<dyn Runnable + Send>;
type SeqRunnable = Box<dyn Runnable>;

#[derive(Debug, Default)]
pub struct Scheduler {
  stages: Vec<Stage>,
}

impl Scheduler {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn run(&mut self, resources: &Resources, thread_pool: &ThreadPool) {
    for stage in &mut self.stages {
      match stage {
        Stage::Seq { runnables } => {
          for runnable in runnables {
            runnable.run(resources, thread_pool);
          }
        }

        Stage::Par { runnables, .. } => thread_pool.install(|| {
          use rayon::iter::{IntoParallelRefMutIterator as _, ParallelIterator as _};

          runnables.par_iter_mut().for_each(|runnable| {
            runnable.run(resources, thread_pool);
          });
        }),
      }
    }
  }

  pub fn add(&mut self, runnable: impl Runnable + Send + 'static) {
    let mut reads = FnvHashSet::default();
    let mut writes = FnvHashSet::default();

    runnable.reads(&mut reads);
    runnable.writes(&mut writes);

    let runnable = Box::new(runnable);
    let mut selected = None;

    for i in (0..self.stages.len()).rev() {
      match &self.stages[i] {
        Stage::Par {
          reads: r,
          writes: w,
          ..
        } if reads.is_disjoint(w) && writes.is_disjoint(r) && writes.is_disjoint(w) => {
          selected = Some(i);
        }

        _ => break,
      }
    }

    if let Some(index) = selected {
      match &mut self.stages[index] {
        Stage::Par {
          reads: r,
          writes: w,
          runnables,
          ..
        } => {
          r.extend(reads.iter());
          w.extend(writes.iter());
          runnables.push(runnable);
        }

        _ => panic!("Cannot add a ParRunnable to a Stage::Seq."),
      }
    } else {
      self.stages.push(Stage::Par {
        reads,
        writes,
        runnables: vec![runnable],
      });
    }
  }

  pub fn add_seq(&mut self, runnable: impl Runnable + 'static) {
    let runnable = Box::new(runnable);

    match self.stages.last_mut() {
      Some(Stage::Seq { runnables }) => {
        runnables.push(runnable);
      }

      _ => self.stages.push(Stage::Seq {
        runnables: vec![runnable],
      }),
    }
  }
}

#[derive(Debug)]
enum Stage {
  Par {
    reads: FnvHashSet<ResourceId>,
    writes: FnvHashSet<ResourceId>,
    runnables: Vec<ParRunnable>,
  },
  Seq {
    runnables: Vec<SeqRunnable>,
  },
}

pub trait Runnable: fmt::Debug {
  fn run(&mut self, resources: &Resources, thread_pool: &ThreadPool);

  fn reads(&self, set: &mut FnvHashSet<ResourceId>);
  fn writes(&self, set: &mut FnvHashSet<ResourceId>);
}

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
