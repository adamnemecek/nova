#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::ecs::{self, ResourceFetch};
use nova::graphics::{self, BackendInstanceExt, PhysicalDeviceExt};
use nova::log;
use nova::thread::ThreadPoolBuilder;
use nova::time;

pub fn main() {
  log::set_as_default().ok();

  let log = log::Logger::new("tvb");

  let pool = &ThreadPoolBuilder::new()
    .build()
    .expect("thread pool creation failed");

  let res = &mut ecs::Resources::new();

  ecs::init(res);

  let systems = &mut ecs::seq![time::Ticker::new(time::Duration::from_hz(60)),];

  ecs::setup(res, systems);
  ecs::dispatch(res, systems, pool);

  log
    .info("Tick.")
    .with("delta", time::Time::fetch(res).delta);

  let instance = graphics::BackendInstance::create("tvb", 1);
  let adapters = instance.enumerate_adapters();
  let mut id = 0;

  for (i, adapter) in adapters.iter().enumerate() {
    log
      .info("Detected adapter.")
      .with("id", i)
      .with("info", &adapter.info)
      .with("limits", adapter.physical_device.limits())
      .with("features", adapter.physical_device.features());

    if let graphics::DeviceType::DiscreteGpu = adapter.info.device_type {
      id = i;
    }
  }

  let adapter = &adapters[id];

  log.info("Selected adapter.").with("id", id);
}
