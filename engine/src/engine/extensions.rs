use super::Context;

pub trait Extension {
  // Invoked before an engine tick.
  fn before_tick(&mut self, _ctx: &mut Context) {}
  // Invoked during an engine tick before systems are dispatched.
  fn before_systems(&mut self, _ctx: &mut Context) {}
  // Invoked during an engine tick after systems are dispatched.
  fn after_systems(&mut self, _ctx: &mut Context) {}
  // Invoked after an engine tick.
  fn after_tick(&mut self, _ctx: &mut Context) {}
}
