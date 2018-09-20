use super::*;

/// Trait that can be implemented by a type for easy engine setup.
pub trait Application: Sized {
  /// Invoked once by `App::run` to set up the world and systems.
  fn setup<'a, 'b>(&mut self, _world: &mut World, _systems: &mut DispatcherBuilder<'a, 'b>) {}

  /// Invoked once per frame by `App::run` before dispatching systems.
  fn before_update(&mut self, _world: &mut World) {}

  /// Invoked once per frame by `App::run` after dispatching systems.
  fn update(&mut self, _world: &mut World) {}

  /// Gets whether to continue running. `App::run` will not exit until this
  /// returns `false`.
  fn is_running(&self) -> bool {
    true
  }

  /// Runs the application until `App::is_running` returns `true`.
  fn run(mut self) {
    // Set up world and systems with `App::setup`.
    let mut world = World::new();
    let mut systems = DispatcherBuilder::new();

    self.setup(&mut world, &mut systems);

    let mut systems = systems.build();

    // Until `App::is_running` is `false`…
    while self.is_running() {
      // Run early update logic before systems.
      self.before_update(&mut world);

      // Dispatch systems.
      systems.dispatch(&mut world.res);

      // Run normal update logic after systems.
      self.update(&mut world);
    }
  }
}
