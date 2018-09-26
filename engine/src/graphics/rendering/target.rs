// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend;
use super::backend::Backend;
use super::Renderer;
use crate::prelude::*;
use gfx_hal as hal;
use gfx_hal::{Device, Surface};

const COLOR_RANGE: hal::image::SubresourceRange = hal::image::SubresourceRange {
  aspects: hal::format::Aspects::COLOR,
  levels: 0..1,
  layers: 0..1,
};

pub struct RenderTarget {
  pub(super) frame_fence: backend::Fence,
  pub(super) frame_semaphore: backend::Semaphore,
  pub(super) render_pass: backend::RenderPass,
  pub(super) swapchain: backend::Swapchain,
  pub(super) extent: hal::window::Extent2D,
  pub(super) frame_views: Vec<(backend::Image, backend::ImageView)>,
  pub(super) framebuffers: Vec<backend::Framebuffer>,
  pub(super) pipeline: backend::GraphicsPipeline,
}

impl RenderTarget {
  pub fn new(renderer: &mut Renderer, size: Vector2<u32>) -> Self {
    let frame_fence = renderer.device.create_fence(false);
    let frame_semaphore = renderer.device.create_semaphore();

    // Get the available capabilities, color formats, and present modes.
    let (surface_caps, surface_formats, present_modes) = renderer
      .surface
      .compatibility(&renderer.adapter.physical_device);

    // Find a good SRGB color format, preferably `Rgba8Srgb`, or default to the
    // first available format.
    let surface_format = select_surface_format(surface_formats);

    // Create a render pass.
    let render_pass = create_render_pass(renderer, surface_format);

    // Create a swapchain config from the caps and selected color format and
    // store its extent.
    let mut swapchain_config = hal::SwapchainConfig::from_caps(&surface_caps, surface_format);

    // Select best available present mode.
    swapchain_config.present_mode = select_present_mode(present_modes);

    // Set extent based on `size` parameter.
    swapchain_config.extent = hal::window::Extent2D {
      width: size.x,
      height: size.y,
    };

    // Store extent for reference while rendering.
    let extent = swapchain_config.extent;

    // If there's space, add one extra image to the swapchain config for
    // triple-buffering.
    //
    // TODO: Is this needed? Should I only do this if the mode is mailbox?
    if surface_caps.image_count.end > swapchain_config.image_count {
      swapchain_config.image_count += 1;
    }

    // Create a swapchain and backbuffer from the swapchain config.
    let (swapchain, backbuffer) =
      renderer
        .device
        .create_swapchain(&mut renderer.surface, swapchain_config, None);

    // Extract the images from the backbuffer and make them into framebuffers.
    let (frame_views, framebuffers) = match backbuffer {
      hal::Backbuffer::Images(images) => {
        let frame_views = images
          .into_iter()
          .map(|image| {
            let image_view = renderer
              .device
              .create_image_view(
                &image,
                hal::image::ViewKind::D2,
                surface_format,
                hal::format::Swizzle::NO,
                COLOR_RANGE.clone(),
              ).expect("could not create image view");
            (image, image_view)
          }).collect::<Vec<_>>();

        let framebuffers = frame_views
          .iter()
          .map(|&(_, ref image_view)| {
            renderer
              .device
              .create_framebuffer(&render_pass, Some(image_view), extent.to_extent())
              .unwrap()
          }).collect();

        (frame_views, framebuffers)
      }

      hal::Backbuffer::Framebuffer(framebuffers) => (Vec::new(), vec![framebuffers]),
    };

    // Create the graphics pipeline.
    let pipeline = create_pipeline(renderer, &render_pass);

    RenderTarget {
      frame_fence,
      frame_semaphore,
      render_pass,
      swapchain,
      extent,
      frame_views,
      framebuffers,
      pipeline,
    }
  }

  pub fn destroy(self, renderer: &Renderer) {
    for framebuffer in self.framebuffers {
      renderer.device.destroy_framebuffer(framebuffer);
    }

    for (_, image_view) in self.frame_views {
      renderer.device.destroy_image_view(image_view);
    }

    renderer.device.destroy_swapchain(self.swapchain);
    renderer.device.destroy_render_pass(self.render_pass);
    renderer.device.destroy_semaphore(self.frame_semaphore);
    renderer.device.destroy_fence(self.frame_fence);
  }
}

fn select_surface_format(formats: Option<Vec<hal::format::Format>>) -> hal::format::Format {
  formats.map_or(hal::format::Format::Rgba8Srgb, |formats| {
    formats
      .iter()
      .find(|format| format.base_format().1 == hal::format::ChannelType::Srgb)
      .map(|format| *format)
      .unwrap_or(formats[0])
  })
}

fn select_present_mode(modes: Vec<hal::window::PresentMode>) -> hal::window::PresentMode {
  // `Fifo` is regular, blocking vsync. It's always available but caps the
  // speed of the engine.
  let mut best = hal::window::PresentMode::Fifo;

  for mode in modes {
    // `Mailbox` allows the engine to run as fast as possible but prevents
    // screen tearing with at least one extra buffer (triple-buffering).
    if mode == hal::window::PresentMode::Mailbox {
      return mode;
    }

    // `Immediate` allows the engine to run as fast as possible and does not
    // prevent screen tearing. Apparently FIFO support isn't great however so
    // immediate is preferred.
    //
    // TODO: Verify that this is true (it was about Vulkan).
    if mode == hal::window::PresentMode::Immediate {
      best = mode;
    }
  }

  return best;
}

fn create_render_pass(
  renderer: &Renderer,
  surface_format: hal::format::Format,
) -> backend::RenderPass {
  // Struct describing an image attached to the render pass.
  let color_attachment = hal::pass::Attachment {
    // Use the given surface format.
    format: Some(surface_format),
    // Don't need more than one sample yet.
    //
    // TODO: Do I ever?
    samples: 1,
    // Clear the image before rendering, store it after so it can be
    // displayed.
    ops: hal::pass::AttachmentOps::new(
      hal::pass::AttachmentLoadOp::Clear,
      hal::pass::AttachmentStoreOp::Store,
    ),
    // Don't use a stencil.
    //
    // TODO: Can I use depth testing for 2D? How does that affect UI?
    stencil_ops: hal::pass::AttachmentOps::DONT_CARE,
    // Don't care what the old layout of an image is (since it gets cleared),
    // but the final layout should be present-ready.
    layouts: hal::image::Layout::Undefined..hal::image::Layout::Present,
  };

  // Struct describing the single subpass of the render pass.
  let subpass = hal::pass::SubpassDesc {
    // Use the above image attachment with the optimal layout.
    colors: &[(0, hal::image::Layout::ColorAttachmentOptimal)],
    depth_stencil: None,
    inputs: &[],
    resolves: &[],
    preserves: &[],
  };

  // Struct describing the dependencies of the subpass.
  let dependency = hal::pass::SubpassDependency {
    passes: hal::pass::SubpassRef::External..hal::pass::SubpassRef::Pass(0),
    stages: hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT
      ..hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
    accesses: hal::image::Access::empty()
      ..(hal::image::Access::COLOR_ATTACHMENT_READ | hal::image::Access::COLOR_ATTACHMENT_WRITE),
  };

  renderer
    .device
    .create_render_pass(&[color_attachment], &[subpass], &[dependency])
}

pub fn create_pipeline(
  renderer: &Renderer,
  render_pass: &backend::RenderPass,
) -> backend::GraphicsPipeline {
  // The pipeline layout defines the shape of the data you can send to a shader.
  // This includes the number of uniforms and push constants. We don't need them
  // for now.
  let pipeline_layout = renderer.device.create_pipeline_layout(&[], &[]);

  let vert_entry = hal::pso::EntryPoint::<Backend> {
    entry: "main",
    module: &renderer.default_shaders.vert,
    specialization: Default::default(),
  };

  let fs_entry = hal::pso::EntryPoint::<Backend> {
    entry: "main",
    module: &renderer.default_shaders.frag,
    specialization: Default::default(),
  };

  let shader_entries = hal::pso::GraphicsShaderSet {
    vertex: vert_entry,
    hull: None,
    domain: None,
    geometry: None,
    fragment: Some(fs_entry),
  };

  let subpass = hal::pass::Subpass {
    index: 0,
    main_pass: render_pass,
  };

  let mut pipeline_desc = hal::pso::GraphicsPipelineDesc::new(
    shader_entries,
    hal::Primitive::TriangleList,
    hal::pso::Rasterizer::FILL,
    &pipeline_layout,
    subpass,
  );

  pipeline_desc.blender.targets.push(hal::pso::ColorBlendDesc(
    hal::pso::ColorMask::ALL,
    hal::pso::BlendState::ALPHA,
  ));

  renderer
    .device
    .create_graphics_pipeline(&pipeline_desc, None)
    .expect("could not create graphics pipeline")
}
