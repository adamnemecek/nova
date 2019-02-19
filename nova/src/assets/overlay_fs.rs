// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct OverlayFs {
  pub paths: Vec<PathBuf>,
}

impl OverlayFs {
  pub fn new() -> Self {
    OverlayFs { paths: Vec::new() }
  }
}

impl OverlayFs {
  pub fn open(&self, relative_path: impl Into<PathBuf>) -> io::Result<File> {
    let relative_path = relative_path.into();

    for mut path in self.paths.iter().cloned() {
      path.push(&relative_path);

      match File::open(path) {
        Ok(file) => {
          return Ok(file);
        }

        Err(err) => {
          if err.kind() == io::ErrorKind::NotFound {
            continue;
          }

          return Err(err);
        }
      }
    }

    Err(io::Error::new(
      io::ErrorKind::NotFound,
      "Could not open the given path relative to any paths in the OverlayFS.",
    ))
  }
}

impl Default for OverlayFs {
  fn default() -> Self {
    let mut fs = OverlayFs::new();

    // Adds the `assets/` and `assets-local/` directories from process working
    // directory as the highest priority paths.
    if let Ok(mut path) = env::current_dir() {
      path.push("assets-local");

      fs.paths.push(path.clone());

      path.pop();
      path.push("assets");

      fs.paths.push(path);
    }

    // Adds the `assets/` and `assets-local/` directories from the binary's path
    // as the lowest priority paths.
    if let Ok(mut path) = env::current_exe() {
      path.pop();

      path.push("assets-local");

      fs.paths.push(path.clone());

      path.pop();
      path.push("assets");

      fs.paths.push(path);
    }

    fs
  }
}
