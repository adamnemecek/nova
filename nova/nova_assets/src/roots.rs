// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::quick_error;
use std::env;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct AssetRoots {
  fs_paths: Vec<PathBuf>,
}

impl Default for AssetRoots {
  fn default() -> Self {
    let mut roots = AssetRoots::new();

    // Adds the `assets/` and `assets-local/` directories from the binary's path
    // as the lowest priority paths.
    if let Ok(path) = env::current_exe() {
      let _ = roots.add_fs_path(path.join("assets"));
      let _ = roots.add_fs_path(path.join("assets-local"));
    }

    // Adds the `assets/` and `assets-local/` directories from process working
    // directory as the highest priority paths.
    if let Ok(path) = env::current_dir() {
      let _ = roots.add_fs_path(path.join("assets"));
      let _ = roots.add_fs_path(path.join("assets-local"));
    }

    roots
  }
}

impl AssetRoots {
  pub fn new() -> Self {
    AssetRoots {
      fs_paths: Vec::new(),
    }
  }

  pub fn fs_paths(&self) -> &[PathBuf] {
    &self.fs_paths
  }

  pub fn add_fs_path(&mut self, path: impl AsRef<Path>) -> Result<(), AddFsPathError> {
    let path = path.as_ref().canonicalize()?;

    if path.is_dir() {
      self.fs_paths.push(path);

      Ok(())
    } else {
      Err(AddFsPathError::NotADirectory)
    }
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum AddFsPathError {
    Io(err: io::Error) {
      from()
      display("io error: {}", err)
      cause(err)
    }
    NotADirectory {
      display("path is not a directory")
    }
  }
}
