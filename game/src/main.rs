#![feature(async_await, futures_api, await_macro)]

mod renderer;

use self::renderer::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  nova::log::set_as_default();

  let mut engine = nova::Engine::default();
  let mut renderer = Renderer::new(engine.resources_mut());

  engine.add_fn(nova::engine::Event::Ticked, move |res, _| {
    renderer.render(res);
  });

  engine.run();

  Ok(())
}
