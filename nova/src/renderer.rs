// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod backend;
pub mod device;
pub mod pipeline;
pub mod shader;

mod alloc;
mod buffer;
mod commands;
mod drawing;
mod framebuffer;
mod presenter;
mod render_pass;
mod sync;
mod texture;

use self::device::{QueueExt, QueueFamilyExt};
use self::framebuffer::Framebuffer;
use self::presenter::Presenter;
use self::sync::FrameSync;
use crate::window::Window;

pub use self::backend::Backend;
pub use self::commands::Commands;
pub use self::device::{Device, DeviceExt, Gpu};
pub use self::drawing::{DrawCommands, Drawable};
pub use self::pipeline::{Pipeline, PipelineBuilder, PipelineStage};
pub use self::render_pass::RenderPass;
pub use self::shader::{Shader, ShaderKind, ShaderSet};
pub use self::texture::{Texture, TextureFormat};

pub struct Renderer {
  framebuffer: Option<Framebuffer>,
  presenter: Presenter,
  commands: Commands,
  render_pass: RenderPass,
  frame_sync: FrameSync,
  queue_index: usize,
  gpu: Gpu,
}

impl Renderer {
  pub fn new(window: &Window) -> Self {
    let gpu = Gpu::new();

    let queue_index = gpu
      .queue_families()
      .iter()
      .position(|f| f.supports_graphics())
      .expect("Device does not support graphics commands.");

    let frame_sync = FrameSync::new(gpu.device());
    let render_pass = render_pass::create(gpu.device());
    let presenter = Presenter::new(&window, &gpu);
    let commands = Commands::new(gpu.device(), &gpu.queue_families()[queue_index]);

    Renderer {
      gpu,
      queue_index,
      frame_sync,
      render_pass,
      commands,
      presenter,
      framebuffer: None,
    }
  }

  pub fn device(&self) -> &Device {
    self.gpu.device()
  }

  pub fn render_pass(&self) -> &RenderPass {
    &self.render_pass
  }

  pub fn begin(&mut self) -> &mut Commands {
    self.frame_sync.wait_for_fence(self.gpu.device());
    self.destroy_framebuffer();

    self
      .presenter
      .begin(&self.gpu, &self.frame_sync.backbuffer_semaphore);

    let framebuffer = Framebuffer::new(
      self.gpu.device(),
      &self.render_pass,
      self.presenter.backbuffer(),
    );

    self.commands.begin();

    self
      .commands
      .begin_render_pass(&self.render_pass, &framebuffer);

    self.framebuffer = Some(framebuffer);

    &mut self.commands
  }

  pub fn finish(&mut self) {
    self.commands.finish_render_pass();
    self.commands.finish();

    let queue = self.gpu.queue_mut(self.queue_index);

    unsafe {
      queue.submit(
        gfx_hal::Submission {
          command_buffers: Some(&self.commands.buffer),
          wait_semaphores: Some((
            &self.frame_sync.backbuffer_semaphore,
            PipelineStage::COLOR_ATTACHMENT_OUTPUT,
          )),
          signal_semaphores: Some(&self.frame_sync.render_semaphore),
        },
        Some(&self.frame_sync.fence),
      );
    }

    self
      .presenter
      .finish(&mut self.gpu, &self.frame_sync.render_semaphore);
  }

  pub fn destroy(mut self) {
    self.destroy_framebuffer();

    let device = self.gpu.device();

    self.presenter.destroy(device);
    self.commands.destroy(device);

    unsafe {
      device.destroy_render_pass(self.render_pass);
    }

    self.frame_sync.destroy(device);
  }

  fn destroy_framebuffer(&mut self) {
    let device = self.gpu.device();

    device.wait_idle().expect("Could not wait for device idle");

    if let Some(framebuffer) = self.framebuffer.take() {
      framebuffer.destroy(device);
    }
  }
}
