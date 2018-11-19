// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `hal` module exposes the contents of the `gfx_hal` crate that the
//! engine was built with. Types that are normally generic over a backend type
//! are replaced with type aliases that use the most appropriate backend for the
//! target platform.
//!
//! The `hal::prelude` module exposes traits needed to invoke methods on backend
//! structures with names prefixed by `Abstract` so that they will not conflict
//! with custom types of the same names. It also exposes the `hal` module itself
//! so that `use hal::prelude::*` offers easy access to graphics API.

pub use gfx_hal::*;

use super::Backend;

pub type Adapter = gfx_hal::Adapter<Backend>;

pub mod pso {
  pub use gfx_hal::pso::*;

  use super::Backend;

  pub type EntryPoint<'a> = gfx_hal::pso::EntryPoint<'a, Backend>;
  pub type GraphicsShaderSet<'a> = gfx_hal::pso::GraphicsShaderSet<'a, Backend>;
}

pub mod command {
  pub use gfx_hal::command::*;

  use super::Backend;

  pub type CommandBufferInheritanceInfo<'a> =
    gfx_hal::command::CommandBufferInheritanceInfo<'a, Backend>;
}

pub mod queue {
  pub use gfx_hal::queue::*;

  use super::Backend;

  pub type Queues = gfx_hal::queue::Queues<Backend>;
}
