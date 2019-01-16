#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::ecs;
use nova::log;
use nova::time;
use nova::window;
use std::thread;

const FRAME_TIME: time::Duration = time::Duration::from_hz(60);

pub fn main() {
  let log = log::Logger::new("tvb");

  log::set_as_default().ok();

  let res = &mut ecs::Resources::new();

  ecs::init(res);

  let mut ticker = time::Ticker::new(time::RealTime::new());

  ecs::setup(res, &mut ticker);

  let mut events_loop = window::EventsLoop::new();

  ecs::setup(res, &mut events_loop);

  window::set_title(res, "tvb");
  window::open(res);

  log.info("Initialized.");

  while !window::is_closed(res) {
    ecs::run(res, &mut events_loop);
    ecs::run(res, &mut ticker);

    thread::sleep(FRAME_TIME.into());
  }
}
