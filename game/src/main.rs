#![feature(async_await, futures_api, await_macro)]

use nova::app::App;
use nova::log;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  log::set_as_default();

  let app = App::default();

  app.run();

  Ok(())
}
