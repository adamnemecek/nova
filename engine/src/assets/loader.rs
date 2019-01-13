// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::OverlayFs;
use crate::tasks;
use crossbeam::channel;
use std::fmt;
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
    mut loader: impl FnMut(File) -> Result<R, LoadError> + Send + 'static,
  ) -> tasks::Completion<Result<R, LoadError>> {
    let task = tasks::CompletionSource::<Result<R, LoadError>>::new();
    let future = task.as_future();

    self.jobs.send(Job {
      path: path.into(),
      load: Box::new(move |file| {
        let result = match file {
          Ok(file) => loader(file),
          Err(err) => Err(LoadError::from(err)),
        };

        task.complete(result);
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

#[derive(Debug)]
pub enum LoadError {
  NotFound,
  Io(io::Error),
  Other(Box<dyn std::error::Error + Send>),
}

impl fmt::Display for LoadError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      LoadError::NotFound => write!(f, "File not found in any asset paths."),
      LoadError::Io(err) => write!(f, "{}", err),
      LoadError::Other(err) => write!(f, "{}", err),
    }
  }
}

impl From<io::Error> for LoadError {
  fn from(error: io::Error) -> Self {
    match error.kind() {
      io::ErrorKind::NotFound => LoadError::NotFound,
      _ => LoadError::Io(error),
    }
  }
}

impl From<Box<dyn std::error::Error + Send>> for LoadError {
  fn from(error: Box<dyn std::error::Error + Send>) -> Self {
    LoadError::Other(error)
  }
}
