#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::log;
use nova::process;
use nova::time;

pub fn main() {
  nova::start(run);
}

async fn run(engine: nova::EngineHandle) {
  let log = engine.execute(|ctx| log::fetch_logger(ctx).with_source("tvb"));

  loop {
    let delta_time = engine.execute(time::delta);

    log.trace("Frame.").with("delta_time", delta_time);

    await!(process::next_tick());
  }
}
