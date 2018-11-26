// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::QueueHandle;
use crate::graphics::commands::Commands;
use crate::graphics::present::Backbuffer;
use crate::graphics::render::pipeline::PipelineStage;
use crate::graphics::sync::{Fence, Semaphore};
use std::sync::Arc;

/// A prepared set of commands with optional semaphores for synchronization.
#[derive(Default)]
pub struct Submission {
  /// Commands to execute.
  pub commands: Vec<Commands>,
  /// Semaphores to wait on before executing the specified pipeline stage.
  pub wait_semaphores: Vec<(Arc<Semaphore>, PipelineStage)>,
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
  pub fn wait_for(&mut self, semaphore: &Arc<Semaphore>, pipeline_stage: PipelineStage) {
    self
      .wait_semaphores
      .push((semaphore.clone(), pipeline_stage));
  }

  /// Adds a backbuffer for the submission to wait on.
  ///
  /// The color attachment output pipeline stage will not execute until this
  /// backbuffer is ready.
  pub fn wait_for_output(&mut self, backbuffer: &Backbuffer) {
    self.wait_for(
      backbuffer.semaphore(),
      PipelineStage::COLOR_ATTACHMENT_OUTPUT,
    );
  }

  /// Adds a semaphore for the submission to signal completion with.
  ///
  /// After all of the commands in the submission have executed, the semaphore
  /// will be signaled.
  pub fn signal_finished(&mut self, semaphore: &Arc<Semaphore>) {
    self.signal_semaphores.push(semaphore.clone());
  }

  /// Clears the submission, removing all commands and semaphores.
  pub fn clear(&mut self) {
    self.commands.clear();
    self.wait_semaphores.clear();
    self.signal_semaphores.clear();
  }
}

pub struct FencedSubmission {
  fence: Fence,
  queue: QueueHandle,
  submission: Submission,
}

impl FencedSubmission {
  pub fn new(queue: &QueueHandle) -> Self {
    FencedSubmission {
      fence: Fence::new(queue.device()),
      queue: queue.clone(),
      submission: Submission::new(),
    }
  }

  /// Gets a reference to the list of semaphores that will signaled when the
  /// submission is complete.
  pub fn signal_semaphores(&mut self) -> &[Arc<Semaphore>] {
    &self.submission.signal_semaphores
  }

  /// Adds a set of commands to be executed.
  pub fn add_commands(&mut self, commands: Commands) {
    self.submission.add_commands(commands)
  }

  /// Adds a semaphore for the submission to wait on.
  ///
  /// The given pipeline stage will not execute until this semaphore has been
  /// signaled.
  pub fn wait_for(&mut self, semaphore: &Arc<Semaphore>, pipeline_stage: PipelineStage) {
    self.submission.wait_for(semaphore, pipeline_stage)
  }

  /// Adds a backbuffer for the submission to wait on.
  ///
  /// The color attachment output pipeline stage will not execute until this
  /// backbuffer is ready.
  pub fn wait_for_output(&mut self, backbuffer: &Backbuffer) {
    self.submission.wait_for_output(backbuffer)
  }

  /// Adds a semaphore for the submission to signal completion with.
  ///
  /// After all of the commands in the submission have executed, the semaphore
  /// will be signaled.
  pub fn signal_finished(&mut self, semaphore: &Arc<Semaphore>) {
    self.submission.signal_finished(semaphore)
  }

  /// Clears the submission, removing all commands and semaphores.
  ///
  /// If the submission is currently executing on the device, this function
  /// will block until it has finished.
  pub fn clear(&mut self) {
    self.fence.wait_and_reset();
    self.submission.clear();
  }

  /// Submits the submission to the queue it was created with.
  pub fn submit(&mut self) {
    self
      .queue
      .lock()
      .submit(&self.submission, Some(&self.fence))
  }
}
