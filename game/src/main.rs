#![feature(async_await, futures_api, await_macro)]

use nova::ecs;
use nova::graphics;
use nova::log;
use nova::time;
use nova::window;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  log::set_as_default();

  let mut res = ecs::setup();

  let _gpu = graphics::setup();

  let (_window, events_loop) = window::create(Default::default());

  let thread_pool = nova::ThreadPoolBuilder::new().build()?;

  let mut dispatcher = ecs::Dispatcher::new(
    ecs::seq![window::PollEvents { events_loop }, time::Elapse::default(),],
    &thread_pool,
  );

  dispatcher.setup(&mut res);

  loop {
    dispatcher.dispatch(&res);

    ecs::maintain(&mut res);

    std::thread::sleep(std::time::Duration::from_millis(1));
  }
}
