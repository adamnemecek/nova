#![feature(async_await, futures_api, await_macro)]

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  nova::log::set_as_default();

  let mut engine = nova::Engine::new(Default::default());
  let log = nova::log::Logger::new("tvb");

  engine.add_fn(nova::engine::Event::ClockTimeUpdated, move |res, _| {
    let time = res.fetch::<nova::clock::Time>();

    log.info("Time updated.").with("delta", time.delta);
  });

  engine.run();

  Ok(())
}
