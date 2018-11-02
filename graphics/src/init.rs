use super::backend;
use super::swapchain;
use super::{Context, RenderPass, RenderState, RenderTarget};
use gfx_hal::pool::{CommandPoolCreateFlags, RawCommandPool};
use gfx_hal::{Device, Instance, PhysicalDevice, QueueFamily, Surface};
use smallvec::SmallVec;
use std::sync::Arc;
use winit;

const ENGINE_NAME: &str = "nova";
const ENGINE_VERSION: u32 = 1;

pub fn init(window: &winit::Window, log: &bflog::Logger) -> (Arc<Context>, RenderTarget) {
  let mut log = log.with_src("graphics");

  let instance = backend::Instance::create(ENGINE_NAME, ENGINE_VERSION);

  log
    .trace("Instantiated backend.")
    .with("backend", &backend::NAME)
    .with("engine_name", &ENGINE_NAME)
    .with("engine_version", &ENGINE_VERSION);

  let surface = instance.create_surface(&window);

  log.trace("Created window surface.");

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

  let (adapter, _) = scored_adapters
    .into_iter()
    .next()
    .expect("no adapters found");

  let memory_properties = adapter.physical_device.memory_properties();

  log
    .trace("Selected best device by score.")
    .with("id", &adapter.info.device)
    .with("name", &adapter.info.name);

  let (device, command_queue, command_queue_id) = {
    let family = adapter
      .queue_families
      .iter()
      .filter(|f| f.supports_graphics() && surface.supports_queue_family(f))
      .next()
      .expect("no supported command queue");

    log
      .trace("Selected command queue.")
      .with("id", &family.id().0);

    let mut gpu = adapter
      .physical_device
      .open(&[(family, &[1.0])])
      .expect("device creation error");

    let device = gpu.device;

    let command_queue = gpu
      .queues
      .take_raw(family.id())
      .expect("queue not found")
      .into_iter()
      .next()
      .expect("empty list of queues");

    (device, command_queue, family.id())
  };

  let context = Arc::new(Context {
    _instance: instance,
    adapter,
    memory_properties,
    device,
    log: log.with_src("graphics"),
  });

  log
    .debug("Created context.")
    .with("backend", &backend::NAME)
    .with("device_id", &context.adapter.info.device)
    .with("device_name", &context.adapter.info.name);

  let (_, formats, _) = surface.compatibility(&context.adapter.physical_device);
  let format = select_format(formats);

  log.trace("Selected image format.").with("format", &format);

  let render_pass = RenderPass::new(&context, format);

  log.trace("Created render pass.");

  let mut command_pool = context.device.create_command_pool(
    command_queue_id,
    CommandPoolCreateFlags::TRANSIENT | CommandPoolCreateFlags::RESET_INDIVIDUAL,
  );

  let command_buffers = command_pool.allocate(
    swapchain::MAX_IMAGE_COUNT,
    gfx_hal::command::RawLevel::Primary,
  );

  let mut states = SmallVec::new();

  for command_buffer in command_buffers {
    states.push(RenderState {
      fence: context.device.create_fence(true),
      acquire_semaphore: context.device.create_semaphore(),
      image: 0,
      command_buffer,
      render_semaphore: context.device.create_semaphore(),
    });
  }

  let window_size = window
    .get_inner_size()
    .expect("window was destroyed")
    .to_physical(window.get_hidpi_factor());

  let width = window_size.width.round() as u32;
  let height = window_size.height.round() as u32;

  let render_target = RenderTarget {
    log: log.with_src("graphics::RenderTarget"),
    context: context.clone(),
    surface,
    command_queue,
    format,
    render_pass,
    command_pool: Some(command_pool),
    states,
    current_state: 0,
    swapchain: None,
    images: SmallVec::new(),
    width,
    height,
  };

  (context, render_target)
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
