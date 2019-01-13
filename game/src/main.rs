#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::log::Logger;
use nova::tasks;
use nova::time;
use std::thread;

const FRAME_TIME: time::Duration = time::Duration::from_hz(60);

pub fn main() {
  let engine = nova::create();

  tasks::spawn(&engine, run());

  loop {
    nova::tick(&engine, FRAME_TIME);

    thread::sleep(FRAME_TIME.into());
  }
}

async fn run() {
  let log = Logger::new("tvb");

  loop {
    log.trace("Tick.");

    await!(tasks::next_tick());
  }
}
