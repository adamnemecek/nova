// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova::ecs;
use nova::graphics;
use nova::window;

pub struct Renderer {
  pass: graphics::RenderPass,
  framebuffers: graphics::FramebufferCache,
  backbuffer_ready: graphics::Semaphore,
}

impl Renderer {
  pub fn new(res: &mut ecs::Resources) -> Self {
    let device = res.fetch();
    let pass = graphics::RenderPass::new(&device);
    let framebuffers = graphics::FramebufferCache::new(&pass);

    Renderer {
      pass,
      framebuffers,
      backbuffer_ready: graphics::Semaphore::new(&device),
    }
  }

  pub fn render(&mut self, res: &mut ecs::Resources) {
    let backbuffer = {
      let mut surface = res.fetch_mut::<window::Surface>();

      surface.acquire_backbuffer(&self.backbuffer_ready)
    };

    let framebuffer = self.framebuffers.cached(backbuffer.index(), |fb| {
      fb.set_size(backbuffer.size());
      fb.attach(0, backbuffer.image());
    });

    let mut surface = res.fetch_mut::<window::Surface>();
    let mut queues = res.fetch_mut();

    surface.present_backbuffer(backbuffer, &mut queues, &self.backbuffer_ready);
  }
}
