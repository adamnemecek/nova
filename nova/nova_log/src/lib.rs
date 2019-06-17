// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use slog::*;
pub use slog_scope::GlobalLoggerGuard;

use std::fmt;
use std::sync::Mutex;

lazy_static::lazy_static! {
  /// Shared default `Logger` which logs using the standard `log` crate.
  pub static ref DEFAULT: Logger = slog::Logger::root(slog_stdlog::StdLog.fuse(), o!());

  /// Static storage for slog_scope's global logger guard so that
  /// `set_global_logger` lasts until called again.
  static ref GLOBAL_GUARD: Mutex<Option<GlobalLoggerGuard>> = Mutex::new(None);
}

/// A struct wrapper for log values that formats the value with `fmt::Debug`.
pub struct Debug<T>(pub T);

impl<T: fmt::Debug> Value for Debug<T> {
  fn serialize(&self, _: &Record, key: Key, serializer: &mut Serializer) -> Result<()> {
    serializer.emit_arguments(key, &format_args!("{:?}", self.0))
  }
}

/// A struct wrapper for log values that formats the value with `fmt::Display`.
pub struct Display<T>(pub T);

impl<T: fmt::Display> Value for Display<T> {
  fn serialize(&self, _: &Record, key: Key, serializer: &mut Serializer) -> Result<()> {
    serializer.emit_arguments(key, &format_args!("{}", self.0))
  }
}

/// Sets the given logger as the default global logger for the `log` and `slog`
/// families of crates.
pub fn set_global_logger(logger: &Logger) {
  // Filter out everything but warning and error messages when using the global
  // logger or `log` crate since these messages are likely outside the nova
  // ecosystem.
  let logger = Logger::root(logger.clone().filter_level(Level::Warning).fuse(), o!());

  // Lock the global logger guard and drop the current one if it exists.
  let mut guard = GLOBAL_GUARD
    .lock()
    .unwrap();

  guard.take();

  // Set the new global logger and store the guard.
  *guard = Some(slog_scope::set_global_logger(logger));

  // Expose the global logger to users of the `log` crate.
  let _ = slog_stdlog::init();
}

/// Gets a reference to a shared default `Logger` which logs using the
/// standard `log` crate.
pub fn default() -> &'static Logger {
  &DEFAULT
}

/// Creates a new terminal `Logger` with full formatting.
pub fn terminal_full() -> Logger {
  let drain = slog_term::term_full().fuse();
  let drain = slog_async::Async::new(drain).build().fuse();

  slog::Logger::root(drain, slog::o!())
}

/// Creates a new terminal `Logger` with compact formatting.
pub fn terminal_compact() -> Logger {
  let drain = slog_term::term_compact().fuse();
  let drain = slog_async::Async::new(drain).build().fuse();

  slog::Logger::root(drain, slog::o!())
}
