#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::ecs;
use nova::log;
use nova::thread;
use nova::time;

pub fn main() {
  log::set_as_default().unwrap();

  let mut res = ecs::Resources::new();

  ecs::setup(&mut res);

  let thread_pool = thread::create_pool();
  let mut on_update = ecs::Dispatcher::new(update(), &thread_pool);

  on_update.setup(&mut res);

  loop {
    on_update.dispatch(&res);

    ecs::maintain(&mut res);

    std::thread::sleep(std::time::Duration::from_millis(1));
  }
}

fn update() -> impl for<'a> ecs::Dispatchable<'a> {
  ecs::seq![time::elapse(),]
}
