#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::log::Logger;
use nova::time;

pub fn main() {
  nova::start(run);
}

async fn run(engine: nova::EngineHandle) {
  let log = Logger::new("tvb");

  loop {
    log
      .trace("Frame.")
      .with("delta_time", time::delta_time(&engine));

    await!(time::delay(&engine, time::Duration::from_secs(1)));
  }
}
