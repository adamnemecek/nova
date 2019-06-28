// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::clock;
use crate::engine::Engine;
use crate::graphics::gpu;
use crate::graphics::gpu::sync::Semaphore;
use crate::graphics::images;
use crate::graphics::renderer::{PipelineStage, RenderOptions, Renderer};
use crate::window;
use std::iter;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

pub struct App {
  pub engine: Engine,
  renderer: Renderer,
}

impl Default for App {
  fn default() -> Self {
    App::new()
  }
}

impl App {
  pub fn new() -> Self {
    let mut engine = Engine::new();

    clock::set_up(&mut engine);
    graphics::set_up(&mut engine).expect("Could not set up graphics");
    window::set_up(&mut engine, Default::default());
    input::set_up(&mut engine);

    let renderer = Renderer::new(&engine.resources);

    App { engine, renderer }
  }

  pub fn run(mut self) {
    const MIN_FRAME_TIME: Duration = Duration::from_micros(16666); // Roughly 60 Hz.

    let (image_sem, render_sem) = {
      let gpu = gpu::borrow(&self.resources);

      (Semaphore::new(&gpu), Semaphore::new(&gpu))
    };

    while !window::borrow(&self.resources).close_requested {
      let began = Instant::now();

      let backbuffer = window::borrow_mut(&self.resources)
        .surface_mut()
        .acquire_backbuffer(
          &gpu::borrow(&self.resources),
          &mut images::borrow_mut(&self.resources),
          &image_sem,
        );

      self.tick();

      if let Some(backbuffer) = backbuffer {
        self.renderer.render(
          &self.engine.resources,
          RenderOptions {
            target: backbuffer.image_id(),
            wait_semaphores: iter::once((&image_sem, PipelineStage::COLOR_ATTACHMENT_OUTPUT)),
            signal_semaphores: iter::once(&render_sem),
          },
        );

        window::borrow_mut(&self.resources)
          .surface_mut()
          .present_backbuffer(
            &mut gpu::queues::borrow_mut(&self.resources),
            backbuffer,
            iter::once(&render_sem),
          );
      }

      let duration = Instant::now() - began;

      if duration < MIN_FRAME_TIME {
        spin_sleep::sleep(MIN_FRAME_TIME - duration);
      }
    }

    {
      let gpu = gpu::borrow(&self.engine.resources);
      let mut images = images::borrow_mut(&self.engine.resources);

      self.renderer.destroy(&gpu);

      window::borrow_mut(&self.engine.resources).destroy(&gpu, &mut images);

      image_sem.destroy(&gpu);
      render_sem.destroy(&gpu);
    }

    graphics::destroy(&mut self.engine);
  }
}

impl Deref for App {
  type Target = Engine;

  fn deref(&self) -> &Engine {
    &self.engine
  }
}

impl DerefMut for App {
  fn deref_mut(&mut self) -> &mut Engine {
    &mut self.engine
  }
}
