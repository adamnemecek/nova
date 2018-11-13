use nova::ecs;
use std::time;

const FRAME_WINDOW: f64 = 200.0;

#[derive(Default)]
pub struct Stats {
  pub fps: Option<f64>,
  pub avg_ms: Option<f64>,
  pub total_secs: f64,
}

#[derive(Default)]
pub struct Counter {
  frames: u64,
  prev_time: Option<time::Instant>,
}

impl Counter {
  pub fn new() -> Self {
    Counter::default()
  }
}

impl<'a> ecs::System<'a> for Counter {
  type Data = ecs::WriteResource<'a, Stats>;

  fn setup(&mut self, ctx: &mut ecs::Context) {
    ecs::put_resource(ctx, Stats::default());
  }

  fn run(&mut self, mut stats: Self::Data) {
    let now = time::Instant::now();

    if let Some(prev_time) = self.prev_time {
      let delta = now - prev_time;
      let secs = delta.as_secs() as f64 + delta.subsec_nanos() as f64 * 1e-9;

      stats.total_secs += secs;

      stats.avg_ms = Some(
        stats.avg_ms.unwrap_or(1.0 / 60.0) * ((FRAME_WINDOW - 1.0) / FRAME_WINDOW)
          + secs * FRAME_WINDOW.recip(),
      );

      stats.fps = stats.avg_ms.map(f64::recip);
    }

    self.prev_time = Some(now);
    self.frames += 1;
  }
}
