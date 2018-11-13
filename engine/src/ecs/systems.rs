pub use specs::SystemData;

use super::Context;
use derive_more::*;

#[derive(From)]
pub struct Dispatcher<'a, 'b>(specs::Dispatcher<'a, 'b>);

impl<'a, 'b> Dispatcher<'a, 'b> {
  pub fn new() -> DispatcherBuilder<'a, 'b> {
    DispatcherBuilder::new()
  }

  pub fn dispatch(&mut self, ecs: &mut Context) {
    self.0.dispatch(&mut ecs.world.res);
  }
}

#[derive(From)]
pub struct DispatcherBuilder<'a, 'b>(specs::DispatcherBuilder<'a, 'b>);

impl<'a, 'b> DispatcherBuilder<'a, 'b> {
  pub fn new() -> DispatcherBuilder<'a, 'b> {
    specs::DispatcherBuilder::new().into()
  }

  pub fn system<S>(self, name: &str, dependencies: &[&str], system: S) -> Self
  where
    S: for<'c> System<'c> + Send + 'a,
  {
    self
      .0
      .with(AsSpecsSystem(system), name, dependencies)
      .into()
  }

  pub fn setup(self, ctx: &mut Context) -> Dispatcher<'a, 'b> {
    let mut dispatcher = self.0.build();

    dispatcher.setup(&mut ctx.world.res);
    dispatcher.into()
  }
}

pub trait System<'a> {
  type Data: SystemData<'a>;

  fn setup(&mut self, _ctx: &mut Context) {}

  fn run(&mut self, data: Self::Data);
}

struct AsSpecsSystem<T>(pub T);

impl<'a, T> specs::System<'a> for AsSpecsSystem<T>
where
  T: System<'a>,
{
  type SystemData = T::Data;

  fn setup(&mut self, res: &mut specs::Resources) {
    T::Data::setup(res);
    System::setup(&mut self.0, res.as_mut());
  }

  fn run(&mut self, data: Self::SystemData) {
    System::run(&mut self.0, data);
  }
}
