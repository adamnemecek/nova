#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::assets;
use nova::ecs::prelude::*;
use nova::log;
use nova::time::{self, Time};
use nova::window::{self, Window};
use std::thread;

const FRAME_TIME: time::Duration = time::Duration::from_hz(60);

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  log::set_as_default().ok();

  let log = log::Logger::new("tvb");

  let mut world = World::new();

  world.add_resource(assets::OverlayFs::default());
  world.add_resource(Time::default());

  let mut ticker = time::Ticker::new(time::RealTime::new());
  let mut window = Window::create(window::Options::default())?;

  loop {
    window.update();

    ecs::run(&world.res, &mut ticker);

    log
      .info("Frame.")
      .with("delta_time", world.read_resource::<Time>().delta);

    thread::sleep(FRAME_TIME.into());
  }
}
