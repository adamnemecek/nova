use super::{backend, Backend};
use gfx_hal::{Instance, PhysicalDevice, QueueFamily, Surface};
use parking_lot::Mutex;
use std::sync::Arc;

const ENGINE_NAME: &str = "nova";
const ENGINE_VERSION: u32 = 1;

pub struct Context {
  // Fields in reverse order of allocation so they are dropped in that order.
  graphics_queue_family: gfx_hal::queue::QueueFamilyId,
  graphics_queue: Mutex<backend::CommandQueue>,
  present_queue_family: gfx_hal::queue::QueueFamilyId,
  present_queue: Mutex<backend::CommandQueue>,
  device: backend::Device,
  adapter: gfx_hal::Adapter<Backend>,
  instance: backend::Instance,
  log: bflog::Logger,
}

impl Context {
  pub fn new(window: &winit::Window, log: &bflog::Logger) -> Arc<Self> {
    let mut log = log.with_src("graphics::Context");

    let instance = backend::Instance::create(ENGINE_NAME, ENGINE_VERSION);

    log
      .trace("Instantiated backend.")
      .with("backend", &backend::NAME)
      .with("engine_name", &ENGINE_NAME)
      .with("engine_version", &ENGINE_VERSION);

    let mut scored_adapters = Vec::new();

    for adapter in instance.enumerate_adapters() {
      let discrete = adapter.info.device_type == gfx_hal::adapter::DeviceType::DiscreteGpu;
      let score = if discrete { 1000 } else { 0 };

      log
        .trace("Scored a device.")
        .with("id", &adapter.info.device)
        .with("name", &adapter.info.name)
        .with("discrete", &discrete)
        .with("score", &score);

      scored_adapters.push((adapter, score));
    }

    scored_adapters.sort_by(|a, b| b.1.cmp(&a.1));

    let (mut adapter, _) = scored_adapters
      .into_iter()
      .next()
      .expect("no adapters found");

    log
      .trace("Selected best device by score.")
      .with("id", &adapter.info.device)
      .with("name", &adapter.info.name);

    let (graphics_queue_family, present_queue_family) = {
      let mut graphics_family = None;
      let mut present_family = None;

      for family in &adapter.queue_families {
        if family.supports_graphics() {
          graphics_family = Some(family);
        }

        if surface.supports_queue_family(family) {
          present_family = Some(family);
        }
      }

      (
        graphics_family.expect("no graphics queue"),
        present_family.expect("no present queue"),
      )
    };

    log
      .trace("Selected command queues.")
      .with("graphics", &graphics_queue_family.id().0)
      .with("present", &present_queue_family.id().0);

    let mut gpu = adapter
      .physical_device
      .open(&[
        (graphics_queue_family, &[1.0]),
        (present_queue_family, &[1.0]),
      ]).expect("device creation error");

    let device = gpu.device;

    let graphics_queue_family = graphics_queue_family.id();
    let graphics_queue = gpu
      .queues
      .take_raw(graphics_queue_family)
      .expect("no graphics queues")
      .into_iter()
      .next()
      .expect("empty list of graphics queues");

    let present_queue_family = present_queue_family.id();
    let present_queue = gpu
      .queues
      .take_raw(present_queue_family)
      .expect("no present queues")
      .into_iter()
      .next()
      .expect("empty list of present queues");;

    let (_, formats, _) = surface.compatibility(&adapter.physical_device);
    let format = select_format(formats);

    log.trace("Selected image format.").with("format", &format);

    log
      .debug("Created.")
      .with("backend", &backend::NAME)
      .with("device_id", &adapter.info.device)
      .with("device_name", &adapter.info.name)
      .with("format", &format);

    Arc::new(Context {
      log,
      instance: instance,
      surface: Mutex::new(surface),
      adapter,
      format,
      device,
      graphics_queue_family,
      present_queue_family,
      graphics_queue: Mutex::new(graphics_queue),
      present_queue: Mutex::new(present_queue),
    })
  }

  pub(super) fn instance(&self) -> &backend::Instance {
    &self.instance
  }

  pub(super) fn device(&self) -> &backend::Device {
    &self.device
  }

  pub fn surface(&self) -> &Mutex<backend::Surface> {
    &self.surface
  }

  pub fn format(&self) -> gfx_hal::format::Format {
    self.format
  }

  pub fn adapter(&self) -> &gfx_hal::Adapter<Backend> {
    &self.adapter
  }

  pub fn log(&self) -> &bflog::Logger {
    &self.log
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    self.log.trace("Dropped.");
  }
}

fn select_format(formats: Option<Vec<gfx_hal::format::Format>>) -> gfx_hal::format::Format {
  formats.map_or(gfx_hal::format::Format::Rgba8Srgb, |formats| {
    formats
      .iter()
      .find(|format| format.base_format().1 == gfx_hal::format::ChannelType::Srgb)
      .map(|format| *format)
      .unwrap_or(formats[0])
  })
}
