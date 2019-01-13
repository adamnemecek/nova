#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::assets;
use nova::log::Logger;
use nova::tasks;
use nova::time;
use std::thread;

const FRAME_TIME: time::Duration = time::Duration::from_hz(60);

pub fn main() {
  let engine = nova::create();

  assets::init(&engine, Default::default());

  tasks::spawn(&engine, run(engine.clone()));

  loop {
    nova::tick(&engine, FRAME_TIME);

    thread::sleep(FRAME_TIME.into());
  }
}

async fn run(engine: nova::EngineHandle) {
  let log = Logger::new("tvb");

  let text = await!(assets::load(&engine, "test.txt", |mut file| {
    use std::io::Read;

    let mut string = String::new();

    file.read_to_string(&mut string)?;

    Ok(string)
  }));

  match text {
    Ok(text) => {
      log.info(text);
    }

    Err(err) => {
      log.error("Error loading file.").with("err", err);
    }
  };
}
