// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod device;
pub mod pipeline;
pub mod shader;

mod alloc;
mod backend;
mod buffer;
mod commands;
mod descriptors;
mod framebuffer;
mod images;
mod presenter;
mod render;
mod render_pass;
mod sync;
mod textures;

pub use self::backend::Backend;
pub use self::commands::Commands;
pub use self::device::{Device, DeviceExt, Gpu};
pub use self::pipeline::{Pipeline, PipelineBuilder, PipelineStage};
pub use self::render::Render;
pub use self::render_pass::RenderPass;
pub use self::shader::{Shader, ShaderKind, ShaderSet};
pub use self::textures::{Textures, TextureId};

use self::alloc::Allocator;
use self::device::{QueueExt, QueueFamilyExt};
use self::framebuffer::Framebuffer;
use self::presenter::Presenter;
use self::sync::FrameSync;
use nova_core::engine::{self, Engine};
use std::iter;

pub struct Renderer {
  gpu: Gpu,
  queue_index: usize,
  allocator: Allocator,
  textures: Textures,
  frame_sync: FrameSync,
  render_pass: RenderPass,
  presenter: Presenter,
  commands: Commands,
  transfer_commands: Commands,
  framebuffer: Option<Framebuffer>,
}

impl Renderer {
  pub fn new(engine: &Engine) -> Self {
    let gpu = Gpu::new();

    let queue_index = gpu
      .queue_families()
      .iter()
      .position(|f| f.supports_graphics())
      .expect("Device does not support graphics commands.");

    let mut allocator = Allocator::new(gpu.physical_device());
    let textures = Textures::new(gpu.device(), &mut allocator);
    let frame_sync = FrameSync::new(gpu.device());
    let render_pass = render_pass::create(gpu.device());
    let presenter = Presenter::new(&engine.resources().fetch(), &gpu);
    let commands = Commands::new(gpu.device(), &gpu.queue_families()[queue_index]);
    let transfer_commands = Commands::new(gpu.device(), &gpu.queue_families()[queue_index]);

    Renderer {
      gpu,
      queue_index,
      allocator,
      textures,
      frame_sync,
      render_pass,
      commands,
      presenter,
      transfer_commands,
      framebuffer: None,
    }
  }

  pub fn device(&self) -> &Device {
    self.gpu.device()
  }

  pub fn render_pass(&self) -> &RenderPass {
    &self.render_pass
  }

  pub fn textures(&self) -> &Textures {
    &self.textures
  }

  pub fn begin(&mut self) -> Render {
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

    Render {
      cmd: &mut self.commands,
      textures: &mut self.textures,
    }
  }

  pub fn finish(&mut self, res: &engine::Resources) {
    self.commands.finish_render_pass();
    self.commands.finish();

    self.transfer_commands.begin();

    self.textures.flush_changes(
      res,
      self.gpu.device(),
      &mut self.allocator,
      &mut self.transfer_commands,
    );

    self.transfer_commands.finish();

    let queue = self.gpu.queue_mut(self.queue_index);

    unsafe {
      queue.submit(
        gfx_hal::Submission {
          command_buffers: &[&self.transfer_commands.buffer, &self.commands.buffer][..],
          wait_semaphores: vec![(
            &self.frame_sync.backbuffer_semaphore,
            PipelineStage::COLOR_ATTACHMENT_OUTPUT,
          )],
          signal_semaphores: iter::once(&self.frame_sync.render_semaphore),
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
    self.transfer_commands.destroy(device);
    self.commands.destroy(device);

    unsafe {
      device.destroy_render_pass(self.render_pass);
    }

    self.frame_sync.destroy(device);
    self.textures.destroy(device, &mut self.allocator);
  }

  fn destroy_framebuffer(&mut self) {
    let device = self.gpu.device();

    device.wait_idle().expect("Could not wait for device idle");

    if let Some(framebuffer) = self.framebuffer.take() {
      framebuffer.destroy(device);
    }
  }
}
