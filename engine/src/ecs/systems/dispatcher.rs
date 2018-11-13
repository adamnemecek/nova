// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{System, SystemData};
use crate::ecs::Context;
use derive_more::*;

/// A dispatcher for running systems in parallel on an ECS context.
#[derive(From)]
pub struct Dispatcher<'a, 'b>(specs::Dispatcher<'a, 'b>);

impl<'a, 'b> Dispatcher<'a, 'b> {
  /// Creates a new dispatcher with the returned [`DispatcherBuilder`].
  pub fn new() -> DispatcherBuilder<'a, 'b> {
    DispatcherBuilder::new()
  }

  /// Runs all systems once in parallel on a thread pool.
  pub fn dispatch(&mut self, ecs: &mut Context) {
    self.0.dispatch(&mut ecs.world.res);
  }
}

/// Builder for constructing a [`Dispatcher`].
#[derive(From)]
pub struct DispatcherBuilder<'a, 'b>(specs::DispatcherBuilder<'a, 'b>);

impl<'a, 'b> DispatcherBuilder<'a, 'b> {
  /// Creates a new builder.
  pub fn new() -> DispatcherBuilder<'a, 'b> {
    specs::DispatcherBuilder::new().into()
  }

  /// Adds a system with the given name and dependencies to the dispatcher.
  ///
  /// System names must be unique, and the list of dependencies is a list of
  /// existing system names.
  ///
  /// The system will run after all of its dependencies have finished running.
  pub fn system<S>(self, name: &str, dependencies: &[&str], system: S) -> Self
  where
    S: for<'c> System<'c> + Send + 'a,
  {
    self
      .0
      .with(AsSpecsSystem(system), name, dependencies)
      .into()
  }

  /// Runs `System::setup` for every system in dependency order and then builds
  /// and returns a new [`Dispatcher`].
  pub fn setup(self, ctx: &mut Context) -> Dispatcher<'a, 'b> {
    let mut dispatcher = self.0.build();

    dispatcher.setup(&mut ctx.world.res);
    dispatcher.into()
  }
}

/// A wrapper struct that implements `specs::System` to convert nova systems to
/// specs systems.
///
/// ## Why not just use `specs::System`?
///
/// To avoid that awkward moment in `System::setup` when you have a
/// `&mut shred::Resources` but you want a `&mut specs::World`.
struct AsSpecsSystem<T>(pub T);

impl<'a, T> specs::System<'a> for AsSpecsSystem<T>
where
  T: System<'a>,
{
  type SystemData = T::Data;

  fn setup(&mut self, res: &mut specs::Resources) {
    // Always set up the data.
    T::Data::setup(res);

    System::setup(&mut self.0, res.as_mut());
  }

  fn run(&mut self, data: Self::SystemData) {
    System::run(&mut self.0, data);
  }
}
