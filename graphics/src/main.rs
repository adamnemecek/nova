use gfx_hal::Device;
use smallvec::SmallVec;
use std::sync::Arc;

mod backend;
mod init;
mod pass;
mod pipeline;
mod rendering;
mod shader;
mod swapchain;

pub use self::backend::Backend;
use self::init::init;
use self::pass::RenderPass;
pub use self::pipeline::{Pipeline, ShaderSet};
pub use self::shader::Shader;

pub struct Context {
  device: backend::Device,
  adapter: backend::Adapter,
  _instance: backend::Instance,
  log: bflog::Logger,
}

pub struct RenderTarget {
  width: u32,
  height: u32,
  images: SmallVec<[RenderImage; swapchain::MAX_IMAGE_COUNT]>,
  swapchain: Option<backend::Swapchain>,
  current_state: usize,
  states: SmallVec<[RenderState; swapchain::MAX_IMAGE_COUNT]>,
  command_pool: Option<backend::CommandPool>,
  render_pass: Arc<RenderPass>,
  format: gfx_hal::format::Format,
  graphics_queue: backend::CommandQueue,
  present_queue: backend::CommandQueue,
  surface: backend::Surface,
  context: Arc<Context>,
  log: bflog::Logger,
}

struct RenderState {
  fence: backend::Fence,
  acquire_semaphore: backend::Semaphore,
  image: u32,
  command_buffer: backend::CommandBuffer,
  render_semaphore: backend::Semaphore,
}

struct RenderImage {
  framebuffer: backend::Framebuffer,
  view: backend::ImageView,
  _raw: backend::Image,
}

impl Drop for Context {
  fn drop(&mut self) {
    self.log.trace("Dropped context.");
  }
}

impl Drop for RenderTarget {
  fn drop(&mut self) {
    swapchain::destroy(self);

    let device = &self.context.device;

    for frame in self.states.drain() {
      device.destroy_fence(frame.fence);
      device.destroy_semaphore(frame.acquire_semaphore);
      device.destroy_semaphore(frame.render_semaphore);
    }

    if let Some(command_pool) = self.command_pool.take() {
      device.destroy_command_pool(command_pool);
    }
  }
}

fn main() {
  let sink = bflog::LogSink::new(
    std::io::stdout(),
    bflog::Format::Modern,
    bflog::LevelFilter::Trace,
  );

  let mut log = bflog::Logger::new(&sink).with_src("graphics");

  let mut events_loop = winit::EventsLoop::new();

  let window = winit::WindowBuilder::new()
    .with_title("nova")
    .build(&events_loop)
    .expect("could not create window");

  let (context, mut render_target) = init(&window, &log);

  let pipeline = Pipeline::new(
    &render_target,
    ShaderSet {
      vertex: Shader::new(&context, include_bytes!("shaders/spirv/default.vert.spv")),
      fragment: Shader::new(&context, include_bytes!("shaders/spirv/default.frag.spv")),
    },
  );

  log.trace("Created main pipeline.");

  let mut exiting = false;

  while !exiting {
    events_loop.poll_events(|event| {
      match event {
        winit::Event::WindowEvent { event, .. } => match event {
          winit::WindowEvent::CloseRequested => {
            log.info("Close requested.");

            exiting = true;
          }

          winit::WindowEvent::Resized(size) => {
            let size = size.to_physical(window.get_hidpi_factor());

            log
              .trace("Window resized.")
              .with("width", &size.width)
              .with("height", &size.height);

            swapchain::destroy(&mut render_target);

            swapchain::create(
              &mut render_target,
              size.width.round() as u32,
              size.height.round() as u32,
            );
          }

          _ => {}
        },

        _ => {}
      };
    });

    if !render_target.swapchain.is_some() {
      let size = window
        .get_inner_size()
        .unwrap()
        .to_physical(window.get_hidpi_factor());

      swapchain::create(
        &mut render_target,
        size.width.round() as u32,
        size.height.round() as u32,
      );
    }

    if let Err(_) = rendering::begin(&mut render_target) {
      swapchain::destroy(&mut render_target);
      continue;
    }

    rendering::bind_pipeline(&mut render_target, &pipeline);
    rendering::draw(&mut render_target);

    if let Err(_) = rendering::end(&mut render_target) {
      swapchain::destroy(&mut render_target);
      continue;
    }

    std::thread::yield_now();
  }
}
