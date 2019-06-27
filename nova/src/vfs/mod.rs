// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod context;
mod paths;

pub use self::{context::*, paths::*};

use super::*;
use std::path::{Path as FsPath, PathBuf as FsPathBuf};

/// Returns a new, empty virtual file system context.
pub fn new() -> Context {
  Context::new()
}
