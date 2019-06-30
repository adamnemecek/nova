// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use slog::{
  b, crit, debug, error, info, o, trace, warn, Drain, Error as SerializationError, Key, Level,
  Logger, Record, Result as SerializationResult, Serializer, Value,
};

use super::*;

/// A struct wrapper for log values that formats the value with `fmt::Debug`.
pub struct Debug<T>(pub T);

impl<T: fmt::Debug> Value for Debug<T> {
  fn serialize(&self, _: &Record, key: Key, serializer: &mut Serializer) -> SerializationResult {
    serializer.emit_arguments(key, &format_args!("{:?}", self.0))
  }
}

/// A struct wrapper for log values that formats the value with `fmt::Display`.
pub struct Display<T>(pub T);

impl<T: fmt::Display> Value for Display<T> {
  fn serialize(
    &self,
    _: &Record,
    key: Key,
    serializer: &mut Serializer,
  ) -> Result<(), SerializationError> {
    serializer.emit_arguments(key, &format_args!("{}", self.0))
  }
}

lazy_static! {
  static ref GUARDS: Mutex<Option<(slog_scope::GlobalLoggerGuard, slog_async::AsyncGuard)>> =
    Mutex::new(None);
  static ref LOGGER: RwLock<Option<Logger>> = RwLock::new(None);
}

/// Initializes the logging module.
pub fn init() {
  let decorator = slog_term::term_full();
  let (drain, async_guard) = slog_async::Async::new(decorator.fuse()).build_with_guard();
  let logger = slog::Logger::root(drain.fuse(), o!());

  // Expose the global logger to users of the `log` and `slog_scope` crates.
  let global_guard = slog_scope::set_global_logger(slog::Logger::root(
    logger.clone().filter_level(Level::Warning).fuse(),
    o!(),
  ));

  let _ = slog_stdlog::init();

  // Store the logger and guard.
  *LOGGER.write() = Some(logger.clone());
  *GUARDS.lock() = Some((global_guard, async_guard));
}

/// Returns a new `Logger` based on the default.
pub fn logger() -> Logger {
  LOGGER.read().as_ref().cloned().expect("log::init has not been called")
}

/// Flushes log records and shuts down the logging module.
///
/// This function should be called before exiting the program to ensure that all
/// log records have been output.
pub fn shut_down() {
  GUARDS.lock().take();
}
