use gfx_hal::Device;
use smallvec::SmallVec;
use std::sync::Arc;

mod backend;
mod init;
mod pass;
mod pipeline;
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
  frames: SmallVec<[RenderFrame; swapchain::MAX_IMAGE_COUNT]>,
  swapchain: Option<backend::Swapchain>,
  command_pool: Option<backend::CommandPool>,
  render_pass: Arc<RenderPass>,
  format: gfx_hal::format::Format,
  graphics_queue_family: gfx_hal::queue::QueueFamilyId,
  graphics_queue: backend::CommandQueue,
  present_queue_family: gfx_hal::queue::QueueFamilyId,
  present_queue: backend::CommandQueue,
  surface: backend::Surface,
  context: Arc<Context>,
  log: bflog::Logger,
}

struct RenderFrame {
  buffer: backend::Framebuffer,
  view: backend::ImageView,
  _image: backend::Image,
  fence: backend::Fence,
  acquire_semaphore: backend::Semaphore,
  render_semaphore: backend::Semaphore,
  command_buffer: backend::CommandBuffer,
}

impl Drop for Context {
  fn drop(&mut self) {
    self.log.trace("Dropped context.");
  }
}

impl Drop for RenderTarget {
  fn drop(&mut self) {
    let device = &self.context.device;

    while let Some(frame) = self.frames.pop() {
      device.destroy_framebuffer(frame.buffer);
      device.destroy_image_view(frame.view);
      device.destroy_fence(frame.fence);
      device.destroy_semaphore(frame.acquire_semaphore);
      device.destroy_semaphore(frame.render_semaphore);
    }

    if let Some(swapchain) = self.swapchain.take() {
      device.destroy_swapchain(swapchain);

      self.log.trace("Destroyed swapchain.");
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

  let mut log = bflog::Logger::new(&sink);

  let mut events_loop = winit::EventsLoop::new();

  let window = winit::WindowBuilder::new()
    .with_title("nova")
    .build(&events_loop)
    .expect("could not create window");

  let (context, render_target) = init(&window, &log);

  let _pipeline = Pipeline::new(
    &render_target,
    ShaderSet {
      vertex: Shader::new(&context, include_bytes!("shaders/spirv/default.vert.spv")),
      fragment: Shader::new(&context, include_bytes!("shaders/spirv/default.frag.spv")),
    },
  );

  log.trace("Created main pipeline.");

  events_loop.run_forever(|event| {
    match event {
      winit::Event::WindowEvent { event, .. } => match event {
        winit::WindowEvent::CloseRequested { .. } => {
          log.info("Close requested.");

          return winit::ControlFlow::Break;
        }

        _ => {}
      },

      _ => {}
    };

    winit::ControlFlow::Continue
  });
}
