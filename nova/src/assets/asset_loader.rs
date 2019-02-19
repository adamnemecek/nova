// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::OverlayFs;
use crate::utils::quick_error;
use crossbeam::channel;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

pub type LoadResult<T> = std::result::Result<T, LoadError>;
pub type LoadReceiver<T> = channel::Receiver<LoadResult<T>>;

#[derive(Debug)]
pub struct AssetLoader {
  fs: Arc<OverlayFs>,
  thread_pool: ThreadPool,
}

impl Default for AssetLoader {
  fn default() -> Self {
    AssetLoader::new(OverlayFs::default())
  }
}

impl AssetLoader {
  pub fn new(fs: impl Into<Arc<OverlayFs>>) -> Self {
    let thread_pool = ThreadPoolBuilder::new()
      .build()
      .expect("Could not create asset loader thread pool");

    AssetLoader {
      fs: fs.into(),
      thread_pool,
    }
  }

  pub fn load<T, P, L>(&self, path: P, loader: L) -> LoadReceiver<T>
  where
    T: Send + 'static,
    P: Into<PathBuf>,
    L: FnOnce(PathBuf, &OverlayFs) -> LoadResult<T> + Send + 'static,
  {
    let path = path.into();
    let fs = self.fs.clone();

    let (sender, receiver) = channel::bounded(1);

    self.thread_pool.spawn(move || {
      // Ignore error because it means the receiver was dropped and this asset
      // is no longer needed.
      let _ = sender.send(loader(path, &fs));
    });

    receiver
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum LoadError {
    Io(err: io::Error) {
      from()
      display("io error: {}", err)
      cause(err)
    }
    Other(err: Box<dyn std::error::Error + Send>) {
      from()
      display("{}", err)
      cause(err.as_ref())
    }
  }
}
