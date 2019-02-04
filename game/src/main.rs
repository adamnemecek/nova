#![feature(async_await, futures_api, await_macro)]

mod renderer;

use self::renderer::*;
use nova::engine::{self, Engine, Event};
use nova::graphics;
use nova::log;
use nova::window;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  nova::log::set_as_default();

  let mut engine = Engine::default();
  let mut renderer = Renderer::new(engine.resources_mut());

  engine.add_fn(Event::Ticked, {
    let device = engine.resources().fetch();
    let queues = engine.resources().fetch();
    let window = engine.resources().fetch();

    let fence = graphics::Fence::new(&device);
    let image_semaphore = graphics::Semaphore::new(&device);
    let render_semaphore = graphics::Semaphore::new(&device);

    let presenter = window::Presenter::new(&window, &queues);

    move |res, _| {
      fence.wait_and_reset();

      presenter.begin(res, &image_semaphore);

      presenter.finish(res, Some(&render_semaphore));
    }
  });

  engine.run();

  Ok(())
}
