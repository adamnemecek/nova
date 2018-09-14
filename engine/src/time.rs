/// Resource that keeps track of elapsed time.
#[derive(Default, Debug)]
pub struct Clock {
  /// Number of frames elapsed since the engine started.
  pub frame: u64,
  /// Time elapsed in seconds since the engine started.
  pub time: f64,
  /// Time elapsed in seconds since the last frame.
  pub delta_time: f64,
}

impl Clock {
  /// Updates the clock with one frame of the given delta time.
  pub fn tick(&mut self, delta_time: f64) {
    self.frame += 1;
    self.delta_time = delta_time;
    self.time += delta_time;
  }
}
