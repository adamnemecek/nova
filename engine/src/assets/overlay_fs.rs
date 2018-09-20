// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::env;
use std::fs::File;
use std::io;

/// Overlay file system for loading and saving files from one or more root
/// paths.
pub struct OverlayFs {
  /// Root paths to use when opening or creating files, in order from first to
  /// last path tried.
  pub root_paths: Vec<PathBuf>,
}

impl OverlayFs {
  /// Loads an asset from a file at the given path in the overlay file system.
  pub fn load<T: Asset>(&self, path: &Path) -> Result<T, Error> {
    T::load(self, path)
  }

  /// Saves an asset to a file at the given path in the overlay file system.
  pub fn save<T: SaveableAsset>(&self, path: &Path, value: T) -> Result<(), Error> {
    value.save(self, path)
  }

  /// Opens the file at the given path in the overlay file system.
  pub fn open(&self, path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref();

    if self.root_paths.len() == 0 {
      return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "assets::Vfs has no paths",
      ));
    }

    for root_path in &self.root_paths {
      let path = root_path.join(path);

      if let Ok(file) = File::open(path) {
        return Ok(file);
      }
    }

    Err(io::Error::new(
      io::ErrorKind::Other,
      format!(
        "assets::OverlayFs could not open {:?} in any of its paths",
        path
      ),
    ))
  }

  /// Creates a file at the given path in the VFS.
  ///
  /// The file is created in the first root path where file creation succeeds.
  pub fn create(&self, path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref();

    if self.root_paths.len() == 0 {
      return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "assets::Vfs has no paths",
      ));
    }

    for root_path in &self.root_paths {
      let path = root_path.join(path);

      if let Ok(file) = File::create(path) {
        return Ok(file);
      }
    }

    Err(io::Error::new(
      io::ErrorKind::Other,
      format!(
        "assets::Vfs could not create {:?} in any of its paths",
        path
      ),
    ))
  }
}

// Sets the default overlay file system to load files from the `assets`
// directory.
impl Default for OverlayFs {
  fn default() -> Self {
    let mut root_paths = Vec::new();

    // Otherwise use the `assets` directory in the exe's directory.
    let mut path = env::current_exe().expect("could not get current exe path");

    path.push("assets");

    root_paths.push(path);

    // If `CARGO_MANIFEST_DIR` is set, use both the `assets` directory and the
    // `assets-local` directory from the directory containing `Cargo.toml`.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
      let mut path = PathBuf::from(manifest_dir);

      path.push("assets");
      root_paths.insert(0, path.clone());
      path.pop();
      path.push("assets-local");
      root_paths.insert(0, path);
    }

    OverlayFs { root_paths }
  }
}

/// Loads an asset from a file at the given path in the overlay file system of
/// the given world.
pub fn load<T: Asset>(world: &mut World, path: &Path) -> Result<T, assets::Error> {
  world.read_resource::<OverlayFs>().load(path)
}
