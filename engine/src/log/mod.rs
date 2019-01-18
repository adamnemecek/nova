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
pub use log::{debug, error, info, trace, warn, SetLoggerError};

use self::color::*;

/// Makes this `log` module the default log handler.
pub fn set_as_default() -> Result<(), SetLoggerError> {
  let logger = Box::new(Logger::new(""));

  log::set_max_level(logger.max_level);
  log::set_boxed_logger(logger)
}
