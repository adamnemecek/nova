use nova::log;
use nova::App;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  // Set up log macros to use nova logging.
  log::set_as_default();

  let app = App::new();

  app.run();

  Ok(())
}
