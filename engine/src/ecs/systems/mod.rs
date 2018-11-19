// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod dispatcher;

pub use self::dispatcher::{Dispatcher, DispatcherBuilder};
pub use specs::SystemData;

use crate::Engine;

/// An ECS system that can be run on a set of resources.
///
/// Systems are typically added to a [`Dispatcher`] which will run them in
/// parallel according to resource requirements.
pub trait System<'a> {
  type Data: SystemData<'a>;

  /// Sets up the engine instance for running this system.
  ///
  /// Components required by the system are automatically registered in the
  /// engine instance before this function is called, but all other resources
  /// must be manually added unless they are expected to already exist.
  fn setup(&mut self, _engine: &mut Engine) {}

  /// Runs the system on the given data.
  fn run(&mut self, data: Self::Data);
}
