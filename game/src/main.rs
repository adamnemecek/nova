#![feature(async_await, futures_api, await_macro)]

use nova::log;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  log::set_as_default();

  let engine = nova::Engine::default();

  engine.run_loop();

  Ok(())
}
