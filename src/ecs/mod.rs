// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod components;
mod context;
mod entities;

pub use self::{components::*, context::*, entities::*};
pub use shred::{ReadExpect as Resource, Resource as ResourceLike, WriteExpect as ResourceMut};
pub use specs::storage;

use super::*;
use hibitset::*;
use shred_derive::*;

#[derive(Clone)]
pub struct Handle {
  context: Arc<Mutex<Context>>,
}

impl Handle {
  pub fn lock(&self) -> MutexGuard<Context> {
    self.context.lock()
  }
}

pub fn new() -> Handle {
  let context = Context::new();

  Handle { context: Arc::new(Mutex::new(context)) }
}
