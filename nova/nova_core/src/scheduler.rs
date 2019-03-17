// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod runnable;

pub use self::runnable::Runnable;

use crate::collections::FnvHashSet;
use crate::ecs::resources::{ResourceId, Resources};
use crate::ThreadPool;
use rayon::iter::{IntoParallelRefMutIterator as _, ParallelIterator as _};

type ParRunnable = Box<dyn Runnable + Send>;
type SeqRunnable = Box<dyn Runnable>;

/// Automatically parallelizes systems and other runnable operations based on
/// their resource dependencies.
#[derive(Debug, Default)]
pub struct Scheduler {
  /// Sequential list of stages to run.
  stages: Vec<Stage>,
}

impl Scheduler {
  pub fn new() -> Self {
    Default::default()
  }

  /// Adds an operation.
  ///
  /// It will be automatically scheduled to run in parallel with other
  /// operations it does not conflict with.
  pub fn add(&mut self, runnable: impl Runnable + Send + 'static) {
    // Determine resource dependencies.
    let mut reads = FnvHashSet::default();
    let mut writes = FnvHashSet::default();

    runnable.reads(&mut reads);
    runnable.writes(&mut writes);

    // Find the existing `Par` stage this runnable should be added to.
    //
    // The search begins from the last stage and searches backward, stopping
    // when the stage has read/write conflicts or is a `Seq` stage. This means:
    //
    //   - The new runnable is placed as early as possible.
    //
    //   - `Seq` stages are treated as barriers, so the runnable will be placed
    //     in a stage _after_ the last `Seq` stage.
    //
    //   - If the runnable reads a resource that a stage writes, it will be
    //     placed in the _following_ stage. The logic is that if a runnable
    //     reads from a resource, it probably benefits from more recent
    //     information.
    //
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

    // Either place the runnable in the selected stage or create a new one.
    let runnable = Box::new(runnable);

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

  /// Adds a sequential runnable to the scheduler.
  ///
  /// It will be run after all previously added runnables have finished running.
  /// Any runnables added after it will not run until this it has finished
  /// running.
  pub fn add_seq(&mut self, runnable: impl Runnable + 'static) {
    let runnable = Box::new(runnable);

    // If the last stage is a `Seq`, add the runnable to that. Otherwise, make
    // a new stage.
    match self.stages.last_mut() {
      Some(Stage::Seq { runnables }) => {
        runnables.push(runnable);
      }

      _ => self.stages.push(Stage::Seq {
        runnables: vec![runnable],
      }),
    }
  }

  /// Runs the scheduled runnables.
  pub fn run(&mut self, resources: &Resources, thread_pool: &ThreadPool) {
    for stage in &mut self.stages {
      match stage {
        Stage::Seq { runnables } => {
          for runnable in runnables {
            runnable.run(resources, thread_pool);
          }
        }

        Stage::Par { runnables, .. } => thread_pool.install(|| {
          runnables.par_iter_mut().for_each(|runnable| {
            runnable.run(resources, thread_pool);
          });
        }),
      }
    }
  }
}

/// A single stage in the `Scheduler` containing runnables.
#[derive(Debug)]
enum Stage {
  /// A stage where all runnables run in parallel.
  Par {
    reads: FnvHashSet<ResourceId>,
    writes: FnvHashSet<ResourceId>,
    runnables: Vec<ParRunnable>,
  },

  /// A stage where all runnables run in sequence.
  Seq { runnables: Vec<SeqRunnable> },
}
