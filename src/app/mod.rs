// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod context;
mod settings;

pub use self::{context::*, settings::*};

use super::*;

pub fn start(
  thread_scope: &thread::Scope,
  logger: log::Logger,
  settings: Settings,
) -> Result<Context, StartError> {
  let ecs = Arc::new(RwLock::new(ecs::Context::new()));
  let gfx = Arc::new(gfx::Context::new(logger.clone())?);

  let window = window::open(thread_scope, settings.window)?;

  gfx::renderer::start(&thread_scope, gfx.clone(), &window, logger.clone())?;

  Ok(Context { ecs, gfx, logger, window })
}

/// An error that occurred while creating a new application context.
#[derive(Debug)]
pub enum StartError {
  /// An error occurred during initialization of the graphics context.
  GraphicsInitFailed(gfx::InitError),
  /// An error occurred while opening the application window.
  OpenWindowFailed(window::OpenError),
  /// An error occurred while starting the renderer.
  RendererStartFailed(gfx::renderer::StartError),
}

impl fmt::Display for StartError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      StartError::GraphicsInitFailed(cause) => {
        write!(f, "failed to initialize graphics: {}", cause)
      }

      StartError::OpenWindowFailed(cause) => {
        write!(f, "failed to open application window: {}", cause)
      }

      StartError::RendererStartFailed(cause) => write!(f, "failed to start renderer: {}", cause),
    }
  }
}

// Implement `From` to convert from out of gfx init errors.
impl From<gfx::InitError> for StartError {
  fn from(cause: gfx::InitError) -> Self {
    StartError::GraphicsInitFailed(cause)
  }
}

// Implement `From` to convert from window open errors.
impl From<window::OpenError> for StartError {
  fn from(cause: window::OpenError) -> Self {
    StartError::OpenWindowFailed(cause)
  }
}

// Implement `From` to convert from renderer start errors.
impl From<gfx::renderer::StartError> for StartError {
  fn from(cause: gfx::renderer::StartError) -> Self {
    StartError::RendererStartFailed(cause)
  }
}
