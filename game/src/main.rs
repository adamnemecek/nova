#![feature(async_await, futures_api, await_macro)]

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  nova::log::set_as_default();

  let engine = nova::Engine::new(Default::default());

  engine.run();

  Ok(())
}
