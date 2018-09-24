// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Context, LoopPhase};

/// Trait for processes that run once per engine loop and have exclusive access
/// to the engine context while running.
pub trait Process {
  /// Invoked once per engine loop.
  fn run(&mut self, ctx: &mut Context);
}

/// State of processes running in the engine.
///
/// Stored in `super::Context`.
pub(super) struct ContextProcesses {
  early: Option<Vec<Box<dyn Process>>>,
  normal: Option<Vec<Box<dyn Process>>>,
  late: Option<Vec<Box<dyn Process>>>,
  added: Vec<(LoopPhase, Box<dyn Process>)>,
}

impl Default for ContextProcesses {
  fn default() -> Self {
    ContextProcesses {
      early: Some(Vec::new()),
      normal: Some(Vec::new()),
      late: Some(Vec::new()),
      added: Vec::new(),
    }
  }
}

// Implement `Process` for functions that take a context.
impl<T> Process for T
where
  T: Fn(&mut Context),
{
  fn run(&mut self, ctx: &mut Context) {
    self(ctx);
  }
}

/// Adds a process that runs early each engine loop.
pub fn add_process_early(ctx: &mut Context, process: impl Process + 'static) {
  ctx
    .processes
    .added
    .push((LoopPhase::Early, Box::new(process)));
}

/// Adds a process that runs each engine loop.
pub fn add_process(ctx: &mut Context, process: impl Process + 'static) {
  ctx
    .processes
    .added
    .push((LoopPhase::Normal, Box::new(process)));
}

/// Adds a process that runs late each engine loop.
pub fn add_process_late(ctx: &mut Context, process: impl Process + 'static) {
  ctx
    .processes
    .added
    .push((LoopPhase::Late, Box::new(process)));
}

/// Runs the given phase of processes.
pub(super) fn run_processes(ctx: &mut Context, phase: LoopPhase) {
  // Take the list of processes out of the `Option` for the given phase.
  let option = match phase {
    LoopPhase::Early => &mut ctx.processes.early,
    LoopPhase::Normal => &mut ctx.processes.normal,
    LoopPhase::Late => &mut ctx.processes.late,
  };

  let mut processes = option.take().expect("phase is already running");

  // Remove any newly added processes for this phase from the `added` buffer
  // and add them to the `processes` vec.
  for i in (0..ctx.processes.added.len()).rev() {
    if ctx.processes.added[i].0 == phase {
      let (_, process) = ctx.processes.added.swap_remove(i);

      processes.push(process);
    }
  }

  // Run each of the processes for this phase with exclusive access to the
  // engine context, minus the list of processes taken from the cell.
  for process in &mut processes {
    process.run(ctx);
  }

  // Fill the `Option` for the given phase back with the updated processes.
  let option = match phase {
    LoopPhase::Early => &mut ctx.processes.early,
    LoopPhase::Normal => &mut ctx.processes.normal,
    LoopPhase::Late => &mut ctx.processes.late,
  };

  *option = Some(processes);
}
