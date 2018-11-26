// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate log;

mod color;
mod context;
mod formats;
mod level;
mod logger;

pub use self::context::*;
pub use self::formats::*;
pub use self::level::*;
pub use self::logger::*;
pub use log::{debug, error, info, trace, warn};

use self::color::*;
use crate::ecs;
use crate::Engine;

/// Sets up the engine instance for logging.
///
/// This will add a [`Logger`] resource that can be retrieved with
/// [`get_logger()`].
pub fn setup(engine: &mut Engine) {
  let logger = Logger::default();

  if logger.set_as_default().is_err() {
    logger
      .with_source("nova::log")
      .warn("Could not set the logger as default: a logging implementation has already been initialized. Logging macros will continue to target the existing implementation.");
  }

  engine.put_resource(logger);
}

/// Gets the [`Logger`] resource registered to the engine instance.
pub fn get_logger(engine: &mut Engine) -> &Logger {
  engine.get_resource_mut()
}

/// Fetches the [`Logger`] resource registered to the engine instance.
pub fn fetch_logger(engine: &Engine) -> ecs::FetchResource<Logger> {
  engine.fetch_resource()
}
