// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod pipeline;

mod drawing;
mod framebuffer;
mod pass;

use super::device::{Device, Fence, QueueSubmission, Queues, Semaphore};
use super::{CommandPool, Commands};
use crate::engine;
use crate::utils::Droppable;
use crate::window;

pub use self::drawing::{DrawCommands, Drawable};
pub use self::framebuffer::*;
pub use self::pass::*;
pub use self::pipeline::{Pipeline, PipelineBuilder, PipelineStage};

pub struct Renderer {
  pass: Pass,
  presenter: window::Presenter,
  fence: Fence,
  backbuffer_semaphore: Semaphore,
  render_semaphore: Semaphore,
  framebuffer: Droppable<Framebuffer>,
  commands: Commands,
  drawables: Vec<Box<dyn Drawable>>,
}

impl Renderer {
  pub fn new(res: &mut engine::Resources) -> Self {
    let presenter = window::Presenter::new(res);

    let device = res.fetch();

    let pass = Pass::new(&device);

    let fence = Fence::new(&device);
    let backbuffer_semaphore = Semaphore::new(&device);
    let render_semaphore = Semaphore::new(&device);

    let commands = {
      let queues = res.fetch::<Queues>();

      let pool = CommandPool::new(
        &device,
        queues
          .get_graphics_queue()
          .expect("The device does not suppord graphics commands."),
      );

      pool.acquire()
    };

    Renderer {
      pass,
      presenter,
      fence,
      backbuffer_semaphore,
      render_semaphore,
      framebuffer: Droppable::dropped(),
      commands,
      drawables: Vec::new(),
    }
  }

  pub fn device(&self) -> &Device {
    self.pass.device()
  }

  pub fn pass(&self) -> &Pass {
    &self.pass
  }

  pub fn add(&mut self, draw: impl Drawable + 'static) {
    self.drawables.push(Box::new(draw));
  }

  pub fn render(&mut self, res: &engine::Resources) {
    self.fence.wait_and_reset();

    self.framebuffer.take();
    self.presenter.begin(&self.backbuffer_semaphore);
    self.framebuffer = Framebuffer::new(&self.pass, self.presenter.backbuffer()).into();

    self.commands.begin();

    self
      .commands
      .begin_render_pass(&self.pass, &self.framebuffer);

    for draw in &mut self.drawables {
      draw.draw(DrawCommands::from(&mut self.commands), res);
    }

    self.commands.finish_render_pass();
    self.commands.finish();

    {
      let mut queues = res.fetch_mut::<Queues>();

      queues.submit(QueueSubmission {
        commands: &self.commands,
        wait_semaphores: Some((
          &self.backbuffer_semaphore,
          PipelineStage::COLOR_ATTACHMENT_OUTPUT,
        )),
        signal_semaphores: Some(&self.render_semaphore),
        fence: Some(&self.fence),
      });
    }

    self.presenter.finish(res, Some(&self.render_semaphore));
  }
}
