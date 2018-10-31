use super::backend;
use super::{Context, RenderTarget};
use gfx_hal::pool::{CommandPoolCreateFlags, RawCommandPool};
use gfx_hal::queue::RawCommandQueue;
use gfx_hal::{Device, Surface};
use smallvec::SmallVec;
use std::sync::Arc;
use std::rc::Rc;

pub struct Renderer {
  context: Arc<Context>,
  surface: backend::Surface,
  graphics_queue_family: gfx_hal::queue::QueueFamilyId,
  graphics_queue: backend::CommandQueue,
  present_queue_family: gfx_hal::queue::QueueFamilyId,
  present_queue: backend::CommandQueue,
  command_pool: Option<backend::CommandPool>,
  render_pass: Option<backend::RenderPass>,
  swapchain: Option<backend::Swapchain>,
  frames: SmallVec<[Frame; 3]>,
}

struct Frame {
  image: backend::Image,
  view: backend::ImageView,
  buffer: backend::Framebuffer,
  fence: backend::Fence,
  acquire_semaphore: backend::Semaphore,
  render_semaphore: backend::Semaphore,
  command_buffer: backend::CommandBuffer,
}

impl Renderer {
  pub fn new(context: &Arc<Context>) -> Renderer {
    let device = context.device();
    let mut log = context.log().with_src("graphics::Renderer");

    let surface = context.instance().create_surface(&window);
    let extent = surface.kind().extent();

    log
      .trace("Created window surface.")
      .with("width", &extent.width)
      .with("height", &extent.height);

    let (surface_caps, _, present_modes) =
      surface.compatibility(&context.adapter().physical_device);

    let extent = gfx_hal::window::Extent2D {
      width: cmp::max(
        surface_caps.extents.start.width,
        cmp::min(width, surface_caps.extents.end.width),
      ),
      height: cmp::max(
        surface_caps.extents.start.height,
        cmp::min(height, surface_caps.extents.end.height),
      ),
    };

    let mut command_pool = device.create_command_pool(
      context.graphics_queue_family,
      CommandPoolCreateFlags::TRANSIENT | CommandPoolCreateFlags::RESET_INDIVIDUAL,
    );

    let mut command_buffers =
      command_pool.allocate(frame_count, gfx_hal::command::RawLevel::Primary);

    let mut states = Vec::with_capacity(frame_count);

    for _ in 0..frame_count {
      states.push(State {
        command_buffers: vec![command_buffers.pop().unwrap()],
        fence: device.create_fence(true),
        image_semaphore: device.create_semaphore(),
        render_semaphore: device.create_semaphore(),
        target: None,
        render_image: 0,
      });
    }

    Renderer {
      context: context.clone(),
      command_pool: Some(command_pool),
      states,
      current_state: 0,
    }
  }

  pub fn begin_render(&mut self, target: &mut RenderTarget) {
    let device = self.context.device();
    let state = &mut self.states[self.current_state];
    let command_buffer = &mut state.command_buffers[0];

    device.wait_for_fence(&state.fence, !0);
    device.reset_fence(&state.fence);

    use gfx_hal::command::{CommandBufferFlags, RawCommandBuffer};
    use gfx_hal::Swapchain;

    let image_index = target
      .swapchain
      .as_mut()
      .expect("swapchain was destroyed")
      .acquire_image(!0, gfx_hal::FrameSync::Semaphore(&state.image_semaphore))
      .expect("could not acquire image");

    let framebuffer = &target.framebuffers[image_index as usize];

    command_buffer.begin(CommandBufferFlags::ONE_TIME_SUBMIT, Default::default());

    command_buffer.begin_render_pass(
      target.render_pass.pass(),
      framebuffer,
      gfx_hal::pso::Rect {
        x: 0,
        y: 0,
        w: target.extent.width as i16,
        h: target.extent.height as i16,
      },
      &[gfx_hal::command::ClearValueRaw::from(
        gfx_hal::command::ClearValue::Color(gfx_hal::command::ClearColor::Float([
          1.0, 0.0, 0.0, 1.0,
        ])),
      )],
      gfx_hal::command::SubpassContents::Inline,
    );
  }

  pub fn end_render(&mut self, target: &RenderTarget) {
    let device = self.context.device();
    let state = &mut self.states[self.current_state];

    let mut cmd_buffers = SmallVec::<[_; 1]>::new();

    cmd_buffers.push(&state.command_buffers[0]);

    let mut signal_semaphores = SmallVec::<[backend::Semaphore; 1]>::new();

    let submission = gfx_hal::queue::RawSubmission {
      cmd_buffers,
      wait_semaphores: &[(
        &state.image_semaphore,
        gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
      )],
      signal_semaphores: &[&state.render_semaphore],
    };

    let mut queue = self
      .context
      .graphics_queue
      .lock()
      .expect("could not lock graphics queue");

    unsafe {
      queue.submit_raw(submission, Some(&state.fence));
    }

    let mut queue = self
      .context
      .present_queue
      .lock()
      .expect("could not lock present queue");

    let mut swapchains = SmallVec::<[_; 1]>::new();
    let mut semaphores = SmallVec::<[_; 1]>::new();

    swapchains.push((target.swapchain.as_ref().expect("swapchain was destroyed"), );
    semaphores.push(&state.render_semaphore);

    queue.present(swapchains, semaphores);
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    let device = self.context.device();

    while let Some(state) = self.states.pop() {
      device.destroy_fence(state.fence);
      device.destroy_semaphore(state.image_semaphore);
      device.destroy_semaphore(state.render_semaphore);
    }

    if let Some(command_pool) = self.command_pool.take() {
      device.destroy_command_pool(command_pool);
    }
  }
}

