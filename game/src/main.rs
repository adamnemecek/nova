#![feature(async_await, futures_api, await_macro)]

use nova::ecs;
use nova::graphics;
use nova::log;
use nova::time;
use nova::window;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  log::set_as_default();

  let mut res = ecs::Resources::new();

  ecs::setup(&mut res);
  graphics::setup(&mut res);

  let events_loop = window::EventsLoop::new();

  let _window = window::WindowBuilder::new()
    .with_title("tvb")
    .build(&events_loop)?;

  let thread_pool = nova::ThreadPoolBuilder::new().build()?;

  let dispatch = ecs::seq![window::PollEvents { events_loop }, time::Elapse::default(),];

  let mut dispatcher = ecs::Dispatcher::new(dispatch, &thread_pool);

  dispatcher.setup(&mut res);

  loop {
    dispatcher.dispatch(&res);

    ecs::maintain(&mut res);

    std::thread::sleep(std::time::Duration::from_millis(1));
  }
}
