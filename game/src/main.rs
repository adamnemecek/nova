// TODO: Remove when RLS supports it.
extern crate nova;

use nova::log;
use nova::log::trace;
use std::fmt;

/// Main entry point of the program.
pub fn main() {
  let mut engine = nova::Engine::new();

  trace!("Log integration works.");

  let log = log::get_logger(&mut engine).with_source("game");

  log.trace("Hello.");
  log.debug("Hello.");
  log.info("Hello.");
  log.warn("Hello.");
  log.error("Hello.");
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
