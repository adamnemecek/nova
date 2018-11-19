// TODO: Remove when RLS supports it.
extern crate nova;

use nova::log;
use nova::log::trace;
use std::fmt;

/// Main entry point of the program.
pub fn main() {
  let mut log = log::Logger::new();

  log.source.push_str("game");

  log.set_as_default();

  trace!("Log integration works.");

  log.warn("Warning 1.");

  log.warn("Warning 2.").with("test", Test { x: 4, y: 10 });
}

pub struct Test {
  x: usize,
  y: usize,
}

impl fmt::Debug for Test {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "(x: {}, y: {})", self.x, self.y)
  }
}
