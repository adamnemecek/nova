// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::ecs;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct EngineHandle(Rc<RefCell<ecs::Context>>);

impl EngineHandle {
  pub(crate) fn new(ctx: ecs::Context) -> Self {
    EngineHandle(Rc::new(RefCell::new(ctx)))
  }

  pub fn execute<R>(&self, func: impl FnOnce(&ecs::Context) -> R) -> R {
    let mut engine = self.0.borrow();

    func(&mut engine)
  }

  pub fn execute_mut<R>(&self, func: impl FnOnce(&mut ecs::Context) -> R) -> R {
    let mut engine = self.0.borrow_mut();

    func(&mut engine)
  }
}
