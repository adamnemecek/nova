// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::OverlayFs;
use crate::tasks;
use crossbeam::channel;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

pub struct AssetLoader {
  fs: Arc<OverlayFs>,
  jobs: channel::Sender<Job>,
}

struct Job {
  path: PathBuf,
  load: Box<dyn FnMut(io::Result<File>) + Send>,
}

impl AssetLoader {
  pub fn new(fs: OverlayFs) -> Self {
    let fs = Arc::new(fs);
    let (sender, receiver) = channel::unbounded();

    let loader = AssetLoader {
      fs: fs.clone(),
      jobs: sender,
    };

    thread::spawn(move || {
      while let Some(Job { path, mut load }) = receiver.recv() {
        load(fs.read(&path));
      }
    });

    loader
  }

  pub fn fs(&self) -> &OverlayFs {
    &self.fs
  }

  pub fn load<R: Send + 'static>(
    &self,
    path: impl Into<PathBuf>,
    mut loader: impl FnMut(io::Result<File>) -> R + Send + 'static,
  ) -> tasks::Completion<R> {
    let task = tasks::CompletionSource::<R>::new();
    let future = task.as_future();

    self.jobs.send(Job {
      path: path.into(),
      load: Box::new(move |file| {
        task.complete(loader(file));
      }),
    });

    future
  }
}

impl Default for AssetLoader {
  fn default() -> Self {
    AssetLoader::new(OverlayFs::default())
  }
}
