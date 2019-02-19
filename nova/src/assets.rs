// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod asset_loader;
mod overlay_fs;

pub use self::asset_loader::{AssetLoader, LoadError, LoadReceiver, LoadResult};
pub use self::overlay_fs::OverlayFs;

use crate::el;
use crate::utils::SharedStr;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Asset<T> {
  pub path: SharedStr,
  pub on_load: el::MessageComposer<LoadResult<T>>,
}

impl<T> PartialEq for Asset<T> {
  fn eq(&self, other: &Self) -> bool {
    self.path == other.path && self.on_load == other.on_load
  }
}

pub trait Load: Send + Sized {
  fn load(path: PathBuf, fs: &OverlayFs) -> LoadResult<Self>;
}

impl Load for String {
  fn load(path: PathBuf, fs: &OverlayFs) -> LoadResult<Self> {
    use std::io::Read;

    let mut file = fs.open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
  }
}

impl<T: Load + fmt::Debug + 'static> el::Element for Asset<T> {
  type State = Option<LoadReceiver<T>>;
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    *ctx.state = Some(AssetLoader::load(
      &ctx.resources.fetch(),
      &self.path,
      T::load,
    ));
  }

  fn on_change(&self, _: Self, ctx: el::Context<Self>) -> el::ShouldRebuild {
    self.on_awake(ctx);

    el::ShouldRebuild(true)
  }

  fn build(&self, children: el::spec::Children, ctx: el::Context<Self>) -> el::Spec {
    if let Some(receiver) = ctx.state {
      el::spec(
        el::common::Receive {
          receiver: receiver.clone(),
          on_recv: self.on_load.clone(),
        },
        children,
      )
    } else {
      children.into()
    }
  }
}
