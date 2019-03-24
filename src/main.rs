use nova::engine::Engine;
use nova::graphics;
use nova::log;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Set up log macros to use nova logging.
  log::set_as_default();

  let mut engine = Engine::new();

  graphics::setup(&mut engine.resources)?;

  Ok(())
}
