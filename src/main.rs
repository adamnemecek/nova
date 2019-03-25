use nova::log;
use nova::App;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Set up log macros to use nova logging.
  log::set_as_default();

  let app = App::new();

  app.run();

  Ok(())
}
