// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova::shred::{Resources, SystemData};
use std::any::Any;

pub trait System<'a, T> {
  type Data: SystemData<'a>;

  fn run(&mut self, message: &'a T, data: Self::Data);
}

type SystemRunner<T> = Box<dyn FnMut(&T, &Resources) + Send>;

pub struct SystemDispatcher<T> {
  systems: Vec<SystemRunner<T>>,
}

impl<T> Default for SystemDispatcher<T> {
  fn default() -> Self {
    Self {
      systems: Default::default(),
    }
  }
}

impl<T> SystemDispatcher<T> {
  pub fn add(&mut self, mut system: impl for<'a> System<'a, T> + Send + 'static) {
    self.systems.push(Box::new(move |message, resources| {
      system.run(message, SystemData::fetch(resources))
    }))
  }

  pub fn run(&mut self, message: &T, resources: &Resources) {
    for runner in &mut self.systems {
      runner(message, resources);
    }
  }
}
