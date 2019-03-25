// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod images;
pub mod render;
pub mod surfaces;
pub mod sync;

mod backend;
mod color;
mod commands;
mod gpu;
mod pipelines;

pub use self::backend::Backend;
pub use self::color::Color4;
pub use self::gpu::GpuSetupError;

use nova_core::engine::Engine;

pub fn set_up(engine: &mut Engine) -> Result<(), GpuSetupError> {
  gpu::set_up(&mut engine.resources)?;

  Ok(())
}
