#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::log::Logger;
use nova::process;
use nova::time;
use std::thread;

const FRAME_TIME: time::Duration = time::Duration::from_hz(60);

pub fn main() {
  let engine = nova::init();

  process::spawn(&engine, run());

  loop {
    engine.tick();

    thread::sleep(FRAME_TIME.into());
  }
}

async fn run() {
  let log = Logger::new("tvb");

  loop {
    log.trace("Tick.");

    await!(process::next_tick());
  }
}
