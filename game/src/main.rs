#![feature(async_await, futures_api, await_macro)]

use nova::ecs;
use nova::graphics;
use nova::log;
use nova::thread;
use nova::time;
use nova::window;

pub fn main() {
  log::set_as_default();

  let mut res = ecs::Resources::new();

  ecs::setup(&mut res);
  window::setup(&mut res, Default::default());
  graphics::setup(&mut res);

  let thread_pool = thread::create_pool();
  let mut updater = ecs::Dispatcher::new(update(), &thread_pool);

  updater.setup(&mut res);

  loop {
    updater.dispatch(&res);

    ecs::maintain(&mut res);

    thread::sleep(time::Duration::from_millis(1));
  }
}

fn update() -> impl for<'a> ecs::Dispatchable<'a> {
  ecs::seq![
    window::poll_events(),
    time::elapse(),
    window::acquire_backbuffer(),
    window::present_backbuffer(),
  ]
}
