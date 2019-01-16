#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::ecs::prelude::*;
use nova::log;
use nova::time::{self, Time};
use nova::window::{self, Window};
use std::thread;

pub fn main() {
  log::set_as_default().ok();

  let log = log::Logger::new("tvb");
  let res = &mut ecs::Resources::new();

  ecs::init(res);

  let mut events_loop = window::EventsLoop::new();
  let mut ticker = time::Ticker::new(time::RealTime::new());

  events_loop.setup(res);
  ticker.setup(res);

  Window::fetch_mut(res).set_title("tvb").open();

  log.info("Initialized.");

  while !Window::fetch(res).is_closed() {
    events_loop.run_now(res);
    ticker.run_now(res);

    ecs::maintain(res);

    if Time::fetch(res).delta < time::Duration::from_millis(1) {
      thread::sleep(std::time::Duration::from_millis(1));
    }
  }
}
