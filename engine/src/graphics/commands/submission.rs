// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Commands;
use crate::graphics::pipeline;
use crate::graphics::Semaphore;
use std::sync::Arc;

/// A prepared set of commands with optional semaphores for synchronization.
#[derive(Default)]
pub struct Submission {
  /// Commands to execute.
  pub commands: Vec<Commands>,
  /// Semaphores to wait on before executing the specified pipeline stage.
  pub wait_semaphores: Vec<(Arc<Semaphore>, pipeline::Stage)>,
  /// Semaphores to signal after all commands have executed.
  pub signal_semaphores: Vec<Arc<Semaphore>>,
}

impl Submission {
  /// Creates a new, empty submission.
  pub fn new() -> Submission {
    Submission::default()
  }

  /// Adds a set of commands to be executed.
  pub fn add_commands(&mut self, commands: Commands) {
    self.commands.push(commands);
  }

  /// Adds a semaphore for the submission to wait on.
  ///
  /// The given pipeline stage will not execute until this semaphore has been
  /// signaled.
  pub fn wait_on(&mut self, semaphore: &Arc<Semaphore>, pipeline_stage: pipeline::Stage) {
    self
      .wait_semaphores
      .push((semaphore.clone(), pipeline_stage));
  }

  /// Adds a semaphore for the submission to signal completion with.
  ///
  /// After all of the commands in the submission have executed, the semaphore
  /// will be signaled.
  pub fn signal(&mut self, semaphore: &Arc<Semaphore>) {
    self.signal_semaphores.push(semaphore.clone());
  }

  /// Clears the submission, removing all commands and semaphores.
  pub fn clear(&mut self) {
    self.commands.clear();
    self.wait_semaphores.clear();
    self.signal_semaphores.clear();
  }
}
