/// Number of times a `Clock` has ticked.
pub type Tick = u64;

/// Resource that keeps track of elapsed time.
#[derive(Default, Debug)]
pub struct Clock {
  /// Number of times the clock has ticked. The clock is ticked once per game
  /// loop.
  pub tick: Tick,
  /// Total time elapsed in seconds.
  pub time: f64,
  /// Time elapsed in seconds between this tick and the last.
  pub delta_time: f64,
}

impl Clock {
  /// Updates the clock with one frame of the given delta time.
  pub fn tick(&mut self, delta_time: f64) {
    self.tick += 1;
    self.delta_time = delta_time;
    self.time += delta_time;
  }
}
