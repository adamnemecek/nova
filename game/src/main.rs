#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::log;
use nova::process;

pub fn main() {
  nova::start(run);
}

async fn run(engine: nova::EngineHandle) {
  let log = engine.execute(|ctx| log::fetch_logger(ctx).with_source("tvb"));

  log.info("Hello from async.");

  await!(process::next_tick());

  log.info("Second frame.");
}
