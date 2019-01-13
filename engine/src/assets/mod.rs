// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod loader;
mod overlay_fs;

pub use self::loader::*;
pub use self::overlay_fs::*;
use crate::tasks;
use crate::EngineHandle;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn init(engine: &EngineHandle, fs: OverlayFs) {
  engine.execute_mut(|ctx| {
    ctx.put_resource(AssetLoader::new(fs));
  });
}

pub fn load<R: Send + 'static>(
  engine: &EngineHandle,
  path: impl Into<PathBuf>,
  loader: impl FnMut(File) -> Result<R, LoadError> + Send + 'static,
) -> tasks::Completion<Result<R, LoadError>> {
  engine.execute(|ctx| {
    let asset_loader = ctx.fetch_resource::<AssetLoader>();

    asset_loader.load(path, loader)
  })
}
