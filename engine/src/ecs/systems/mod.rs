// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod dispatcher;

pub use self::dispatcher::{Dispatcher, DispatcherBuilder};
use super::Context;
pub use specs::SystemData;

/// An ECS system that can be run on a set of resources.
///
/// Systems are typically added to a [`Dispatcher`] which will run them in
/// parallel according to resource requirements.
pub trait System<'a> {
  type Data: SystemData<'a>;

  fn setup(&mut self, ctx: &mut Context) {
    Self::setup_data(ctx);
  }

  fn setup_data(ctx: &mut Context) {
    Self::Data::setup(ctx.as_mut());
  }

  /// Runs the system on the given data.
  fn run(&mut self, data: Self::Data);
}
