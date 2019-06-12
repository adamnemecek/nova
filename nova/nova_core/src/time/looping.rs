use crate::time;

#[derive(Default)]
pub struct LoopContext {
  min_time: time::Duration,
  delta_time: time::Duration,
  stop: bool,
}

impl LoopContext {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn min_time(&self) -> time::Duration {
    self.min_time
  }

  pub fn set_frequency(&mut self, value: f64) {
    self.min_time = time::Duration::from_secs(1.0 / value);
  }

  pub fn set_min_time(&mut self, value: time::Duration) {
    self.min_time = value;
  }

  pub fn stop(&mut self) {
    self.stop = true;
  }

  pub fn is_stopping(&self) -> bool {
    self.stop
  }

  pub fn delta_time(&self) -> time::Duration {
    self.delta_time
  }

  pub fn run(&mut self, closure: impl Fn(&mut LoopContext)) {
    self.delta_time = time::Duration::default();

    while !self.stop {
      let began = time::Instant::now();

      closure(self);

      let duration = began.elapsed();

      if duration < self.min_time {
        spin_sleep::sleep((self.min_time - duration).into());
      }

      self.delta_time = began.elapsed();
    }
  }
}

pub fn loop_at_frequency(hz: f64, closure: impl Fn(&mut LoopContext)) {
  let mut ctx = LoopContext::default();

  ctx.set_frequency(hz);
  ctx.run(closure)
}
