use nova::graphics;
use nova::graphics::rendering;
use nova::graphics::window;
use nova::utils::Nullable;
use std::iter;

pub fn main() -> Result<(), String> {
  let mut gfx = graphics::Context::new("nova/examples/boilerplate", 1)
    .map_err(|err| format!("Could not create graphics context: {}", err))?;

  let mut renderer = rendering::Renderer::new(&gfx.queues.graphics);
  let mut swapchain = Nullable::<window::Swapchain>::new();

  let command_pool = rendering::CommandPool::new(&gfx.queues.graphics);

  loop {
    gfx.window.update();

    if gfx.window.is_closed() {
      break;
    }

    if !swapchain.is_null() && swapchain.size() != gfx.window.size() {
      swapchain.drop();
    }

    let (framebuffer, framebuffer_semaphore) = loop {
      if swapchain.is_null() {
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

    let render_semaphore = renderer.render(&framebuffer, &framebuffer_semaphore, iter::once(cmd));

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
