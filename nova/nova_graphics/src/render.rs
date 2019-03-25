// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod renderer;

pub use self::renderer::Renderer;

use crate::images::ImageId;

pub struct RenderOptions<W, S> {
  pub target: ImageId,
  pub wait_for: W,
  pub signal: S,
}
