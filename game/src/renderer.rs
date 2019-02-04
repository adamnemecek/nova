// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova::ecs;
use nova::graphics;
use nova::window;

const FRAMES_IN_FLIGHT: usize = 3;

pub struct Renderer {
  pass: graphics::RenderPass,
  framebuffers: graphics::FramebufferCache,
  commands: Vec<graphics::Commands>,
  backbuffer_semaphore: graphics::Semaphore,
  render_semaphore: graphics::Semaphore,
  frame_fence: graphics::Fence,
}

impl Renderer {
  pub fn new(res: &mut ecs::Resources) -> Self {
    let device = res.fetch();
    let queues = res.fetch::<graphics::Queues>();

    let pass = graphics::RenderPass::new(&device);
    let framebuffers = graphics::FramebufferCache::new(&pass);

    let commands = {
      let queue_id = queues
        .get_graphics_queue()
        .expect("Device does not support graphics commands.");

      let pool = graphics::CommandPool::new(&device, queue_id);

      std::iter::repeat_with(|| pool.acquire())
        .take(FRAMES_IN_FLIGHT)
        .collect()
    };

    Renderer {
      pass,
      framebuffers,
      commands,
      backbuffer_semaphore: graphics::Semaphore::new(&device),
      render_semaphore: graphics::Semaphore::new(&device),
      frame_fence: graphics::Fence::new(&device),
    }
  }

  pub fn render(&mut self, res: &mut ecs::Resources) {
    self.frame_fence.wait_and_reset();

    let backbuffer = {
      let mut surface = res.fetch_mut::<window::Surface>();

      surface.acquire_backbuffer(&self.backbuffer_semaphore)
    };

    let framebuffer = self.framebuffers.cached(backbuffer.index(), |fb| {
      fb.set_size(backbuffer.size());
      fb.attach(0, backbuffer.image());
    });

    let commands = &mut self.commands[backbuffer.index()];

    commands.begin();
    commands.begin_render_pass(&self.pass, &framebuffer);

    commands.finish_render_pass();
    commands.finish();

    {
      let mut queues = res.fetch_mut::<graphics::Queues>();

      queues.submit(graphics::QueueSubmission {
        commands: &commands,
        wait_semaphores: Some((
          &self.backbuffer_semaphore,
          graphics::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
        )),
        signal_semaphores: Some(&self.render_semaphore),
        fence: Some(&self.frame_fence),
      });
    }

    let mut surface = res.fetch_mut::<window::Surface>();
    let mut queues = res.fetch_mut();

    surface.present_backbuffer(backbuffer, &mut queues, &self.render_semaphore);
  }
}
