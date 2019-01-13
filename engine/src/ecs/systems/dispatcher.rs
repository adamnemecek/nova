// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{System, SystemData};
use crate::Context;
use derive_more::*;

/// A dispatcher for running systems in parallel on an ECS context.
#[derive(From)]
pub struct Dispatcher(specs::Dispatcher<'static, 'static>);

impl Dispatcher {
  /// Runs all systems once in parallel on a thread pool.
  pub fn dispatch(&mut self, engine: &mut Context) {
    self.0.dispatch(engine.as_mut());
  }
}

/// Builder for constructing a [`Dispatcher`].
#[derive(From, Default)]
pub struct DispatcherBuilder(specs::DispatcherBuilder<'static, 'static>);

impl DispatcherBuilder {
  /// Creates a new builder.
  pub fn new() -> Self {
    specs::DispatcherBuilder::new().into()
  }

  /// Adds a system with the given name and dependencies to the dispatcher.
  ///
  /// System names must be unique, and the list of dependencies is a list of
  /// existing system names.
  ///
  /// The system will run after all of its dependencies have finished running.
  pub fn add_system<S>(self, name: &str, dependencies: &[&str], system: S) -> Self
  where
    S: for<'a> System<'a> + Send + 'static,
  {
    self
      .0
      .with(AsSpecsSystem(system), name, dependencies)
      .into()
  }

  /// Runs `System::setup` for every system in dependency order and then builds
  /// and returns a new [`Dispatcher`].
  pub fn build(self, engine: &mut Context) -> Dispatcher {
    let mut dispatcher = self.0.build();

    dispatcher.setup(engine.as_mut());
    dispatcher.into()
  }
}

/// A wrapper struct that implements `specs::System` to convert nova systems to
/// specs systems.
struct AsSpecsSystem<T>(pub T);

impl<'a, T> specs::System<'a> for AsSpecsSystem<T>
where
  T: System<'a>,
{
  type SystemData = T::Data;

  fn setup(&mut self, res: &mut specs::Resources) {
    // Always set up the data.
    T::Data::setup(res);

    let engine: &mut Context = res.as_mut();

    System::setup(&mut self.0, engine);
  }

  fn run(&mut self, data: Self::SystemData) {
    System::run(&mut self.0, data);
  }
}
