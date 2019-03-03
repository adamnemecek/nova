// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::ecs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Asset {
  pub(crate) fs_path: PathBuf,
}

impl Asset {
  pub fn fs_path(&self) -> &Path {
    &self.fs_path
  }
}

impl ecs::Component for Asset {
  type Storage = ecs::HashMapStorage<Self>;
}
