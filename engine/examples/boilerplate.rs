use nova::graphics;
use nova::graphics::rendering;
use nova::graphics::window;
use nova::utils::Droppable;
use std::iter;

pub fn main() -> Result<(), String> {
  let mut gfx = graphics::Context::new("nova/examples/boilerplate", 1)
    .map_err(|err| format!("Could not create graphics context: {}", err))?;

  let mut renderer = rendering::Renderer::new(&gfx.queues.graphics);
  let mut swapchain = Droppable::<window::Swapchain>::dropped();

  let command_pool = rendering::CommandPool::new(&gfx.queues.graphics);

  for fence in &mut graphics::device::Fence::chain(&gfx.device, 3) {
    gfx.window.update();

    if gfx.window.is_closed() {
      break;
    }

    if !swapchain.is_dropped() && swapchain.size() != gfx.window.size() {
      swapchain.drop();
    }

    let (framebuffer, framebuffer_semaphore) = loop {
      if swapchain.is_dropped() {
        let size = gfx.window.size();

        swapchain =
          window::Swapchain::new(renderer.pass(), gfx.window.raw_surface_mut(), size).into();
      }

      match swapchain.acquire_framebuffer() {
        Ok(fb) => break fb,
        Err(_) => swapchain.drop(),
      };
    };

    let mut cmd =
      rendering::CommandBuffer::new(&command_pool, rendering::CommandBufferKind::Primary);

    cmd.begin();

    cmd.begin_pass(renderer.pass(), &framebuffer);

    cmd.finish();

    fence.wait();

    let render_semaphore = renderer.render(
      &framebuffer,
      iter::once(cmd),
      &framebuffer_semaphore,
      Some(fence),
    );

    let result = gfx.queues.graphics.present(
      iter::once((swapchain.as_ref(), framebuffer.index())),
      iter::once(render_semaphore),
    );

    if let Err(_) = result {
      swapchain.drop();
    }
  }

  Ok(())
}
