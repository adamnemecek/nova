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
  graphics::setup(&mut res);
  window::setup(&mut res, Default::default());

  let thread_pool = thread::create_pool();

  let dispatch = ecs::seq![window::Update, time::Elapse::new(),];

  let mut dispatcher = ecs::Dispatcher::new(dispatch, &thread_pool);

  dispatcher.setup(&mut res);

  loop {
    dispatcher.dispatch(&res);

    ecs::maintain(&mut res);

    thread::sleep(time::Duration::from_millis(1));
  }
}
