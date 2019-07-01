// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub struct Context {
  pub ecs: Arc<RwLock<ecs::Context>>,
  pub gfx: Arc<gfx::Context>,
  pub logger: log::Logger,
  pub window: window::Handle,
}
