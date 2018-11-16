// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod pool;

pub use self::pool::CommandPool;
pub use gfx_hal::command::RawLevel as Level;

use super::backend;
use super::hal::prelude::*;
use super::pipeline::{DescriptorSet, Pipeline};
use super::{Buffer, Framebuffer, RenderPass};
use crate::utils::Droppable;
use std::iter;
use std::sync::atomic;
use std::sync::Arc;

/// A list of commands to be executed by a graphics device.
pub struct Commands {
  /// The command pool the commands are stored in.
  pool: Arc<CommandPool>,
  /// Raw command buffer.
  buffer: Droppable<backend::CommandBuffer>,
  /// Whether or not commands are being recorded.
  recording: bool,
  /// Render passes referenced by the commands.
  render_passes: Vec<Arc<RenderPass>>,
  /// Pipelines referenced by the commands.
  pipelines: Vec<Arc<Pipeline>>,
  /// Secondary commands executed by the commands.
  children: Vec<Arc<Commands>>,
}

impl Commands {
  /// Creates a new set of commands stored in the given command pool.
  pub fn new(pool: &Arc<CommandPool>, level: Level) -> Commands {
    let buffer = pool.allocate_raw(level);

    Commands {
      pool: pool.clone(),
      buffer: buffer.into(),
      recording: false,
      render_passes: Vec::new(),
      pipelines: Vec::new(),
      children: Vec::new(),
    }
  }

  /// Begins recording commands.
  ///
  /// This will reset any previously recorded commands and clear all referenced
  /// resources.
  pub fn begin(&mut self) {
    self.start_recording();
    self.buffer.begin(Default::default(), Default::default());
  }

  /// Records a command to begin a render pass with the given framebuffer.
  pub fn begin_render_pass(&mut self, render_pass: &Arc<RenderPass>, framebuffer: &Framebuffer) {
    assert!(self.recording, "The command buffer is not recording.");

    // Convert the framebuffer size from `u32` to `i16`.
    let size = framebuffer.size().vector.map(|u| u as i16);

    // Create a viewport struct covering the entire framebuffer.
    let viewport = hal::pso::Viewport {
      rect: hal::pso::Rect {
        x: 0,
        y: 0,
        w: size.x as i16,
        h: size.y as i16,
      },
      depth: 0.0..1.0,
    };

    // Begin the render pass.
    self.record_raw(|buffer| {
      buffer.set_viewports(0, &[viewport.clone()]);
      buffer.set_scissors(0, &[viewport.rect]);

      buffer.begin_render_pass(
        render_pass.raw(),
        framebuffer.as_ref(),
        viewport.rect,
        &[
          // Clear the framebuffer to eigengrau.
          hal::command::ClearValue::Color(hal::command::ClearColor::Float([
            0.086, 0.086, 0.114, 1.0,
          ]))
          .into(),
        ],
        hal::command::SubpassContents::Inline,
      );
    });

    // Store a reference to the render pass so that it isn't dropped.
    self.render_passes.push(render_pass.clone());
  }

  /// Finishes the current render pass.
  pub fn finish_render_pass(&mut self) {
    self.buffer.end_render_pass();
  }

  /// Begins recording a new set of commands that will execute entirely within
  /// the given render pass. Can only be used with secondary commands.
  ///
  /// This will reset any previously recorded commands and clear all referenced
  /// resources.
  pub fn begin_in_render_pass(&mut self, pass: &Arc<RenderPass>, framebuffer: &Framebuffer) {
    self.start_recording();

    self.buffer.begin(
      Default::default(),
      hal::command::CommandBufferInheritanceInfo {
        subpass: Some(hal::pass::Subpass {
          index: 0,
          main_pass: pass.raw(),
        }),
        framebuffer: Some(framebuffer.as_ref()),
        ..Default::default()
      },
    );

    // Store a reference to the render pass so that it isn't dropped.
    self.render_passes.push(pass.clone());
  }

  /// Records commands to execute the given secondary commands.
  pub fn execute_commands(&mut self, commands: impl IntoIterator<Item = Arc<Commands>>) {
    assert!(self.recording, "The command buffer is not recording.");

    // Store a reference to each set of secondary commands so they do not get
    // dropped.
    let start = self.children.len();

    self.children.extend(commands);

    // Record commands for each of the new children.
    self.buffer.execute_commands(
      self.children[start..]
        .iter()
        .map(AsRef::as_ref) // &Arc<Commands> -> &Commands
        .map(AsRef::as_ref), // &Commands -> &backend::CommandBuffer
    );
  }

  /// Records a command to bind the given pipeline.
  ///
  /// Subsequent draw commands will use this pipeline.
  pub fn bind_pipeline(&mut self, pipeline: &Arc<Pipeline>) {
    self
      .buffer
      .bind_graphics_pipeline(pipeline.as_ref().as_ref());

    self.pipelines.push(pipeline.clone());
  }

  /// Records a command to push a constant value.
  pub fn push_constant<T>(&mut self, index: usize, value: &T) {
    assert!(self.recording, "The command buffer is not recording.");

    let pipeline = self
      .pipelines
      .last()
      .expect("A pipeline must be bound to push constant values.");

    let range = pipeline.push_constant_range(index);

    // Convert the constant to a slice of `u32` as vulkan/gfx-hal expects.
    let constants =
      unsafe { std::slice::from_raw_parts(value as *const T as *const u32, range.len()) };

    self.buffer.push_graphics_constants(
      pipeline.raw_layout(),
      gfx_hal::pso::ShaderStageFlags::VERTEX | gfx_hal::pso::ShaderStageFlags::FRAGMENT,
      range.start,
      constants,
    );
  }

  /// Records a command to bind a descriptor set to the given binding index.
  pub fn bind_descriptor_set(&mut self, index: usize, set: &DescriptorSet) {
    assert!(self.recording, "The command buffer is not recording.");

    let pipeline = self
      .pipelines
      .last()
      .expect("A pipeline must be bound to bind a descriptor set.");

    self.buffer.bind_graphics_descriptor_sets(
      pipeline.raw_layout(),
      index,
      iter::once(set.as_ref()),
      &[],
    );
  }

  /// Records a command to bind a vertex buffer to the given binding index.
  pub fn bind_vertex_buffer<T: Copy>(&mut self, binding: u32, buffer: &Buffer<T>) {
    assert!(self.recording, "The command buffer is not recording.");

    assert!(
      self.pipelines.len() > 0,
      "A pipeline must be bound to bind a vertex buffer.",
    );

    self
      .buffer
      .bind_vertex_buffers(binding, iter::once((buffer.as_ref(), 0)));
  }

  /// Records a command to bind an index buffer to the given binding index.
  pub fn bind_index_buffer(&mut self, buffer: &Buffer<u16>) {
    assert!(self.recording, "The command buffer is not recording.");

    assert!(
      self.pipelines.len() > 0,
      "A pipeline must be bound to bind an index buffer.",
    );

    self.buffer.bind_index_buffer(hal::buffer::IndexBufferView {
      buffer: buffer.as_ref(),
      offset: 0,
      index_type: hal::IndexType::U16,
    })
  }

  /// Records a command to draw a given number of indices from the bound index
  /// buffer.
  pub fn draw_indexed(&mut self, indices: u32) {
    assert!(self.recording, "The command buffer is not recording.");

    assert!(
      self.pipelines.len() > 0,
      "A pipeline must be bound to draw indices.",
    );

    self.buffer.draw_indexed(0..indices, 0, 0..1);
  }

  /// Records commands to the underlying buffer with the given function.
  pub fn record_raw<R>(&mut self, f: impl FnOnce(&mut backend::CommandBuffer) -> R) -> R {
    assert!(self.recording, "The command buffer is not recording.");

    f(&mut self.buffer)
  }

  /// Finishes recording commands.
  pub fn finish(&mut self) {
    assert!(self.recording, "The command buffer is not recording.");

    self.buffer.finish();
    self.stop_recording();
  }

  /// Starts recording commands by setting the [`recording`] and
  /// [`CommandPool::recording`] flags.
  fn start_recording(&mut self) {
    if !self.recording {
      let was_recording =
        self
          .pool
          .recording
          .compare_and_swap(false, true, atomic::Ordering::Relaxed);

      if was_recording {
        panic!("Another set of commands in the same command pool is already recording.");
      }

      self.recording = true;
    }

    self.render_passes.clear();
    self.pipelines.clear();
  }

  /// Stops recording commands by unsetting the [`recording`] and
  /// [`CommandPool::recording`] flags.
  fn stop_recording(&mut self) {
    if self.recording {
      self.pool.recording.store(false, atomic::Ordering::Relaxed);
      self.recording = false;
    }
  }
}

// Implement `Drop` to stop recording and free the underlying buffer.
impl Drop for Commands {
  fn drop(&mut self) {
    if self.recording {
      self.stop_recording();
    }

    if let Some(buffer) = self.buffer.take() {
      unsafe {
        self.pool.free_raw(buffer);
      }
    }
  }
}

// Implement `AsRef` to expose a reference to the underlying buffer.
impl AsRef<backend::CommandBuffer> for Commands {
  fn as_ref(&self) -> &backend::CommandBuffer {
    &self.buffer
  }
}
