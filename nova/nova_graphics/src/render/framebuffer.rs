// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::Gpu;
use crate::images::Image;
use crate::render::RenderPass;
use crate::Backend;
use gfx_hal::Device as _;

type HalFramebuffer = <Backend as gfx_hal::Backend>::Framebuffer;

pub struct Framebuffer {
  framebuffer: HalFramebuffer,
}
