// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `assets` module handles loading and saving data from files.
//!
//! The `OverlayFs` resource provides an _overlay file system_ where one or more
//! paths are “layered” on top of one another. Opening a file with the overlay
//! file system checks for the file in each path. This can be used to override
//! game data with different folders.
//!
//! The `Asset` and `SaveableAsset` traits implement loading and saving of files
//! using an overlay file system. Structs and types representing game data or
//! resources can implement these traits for easy loading and saving.

use super::*;
pub use std::path::{Path, PathBuf};

mod asset;
mod overlay_fs;

pub use self::{asset::*, overlay_fs::*};

/// Error returned from asset operations.
pub type Error = Box<dyn std::error::Error>;

/// Sets up asset handling for the given world.
pub fn setup(world: &mut World) {
  world.add_resource(OverlayFs::default());
}
