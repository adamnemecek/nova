// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod context;

mod color;
mod formats;
mod level;
mod logger;

pub use self::color::Color;
pub use self::formats::Display;
pub use self::level::{Level, LevelFilter, PrettyLevel};
pub use self::logger::Logger;
pub use log::{debug, error, info, trace, warn};

/// Makes this `log` module the default log handler.
pub fn set_as_default() {
  let logger = Box::new(Logger::new(""));

  log::set_max_level(logger.max_level);
  log::set_boxed_logger(logger).expect("Could not set nova::log as the default logger");
}
