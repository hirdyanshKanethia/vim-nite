use std::time::Duration;

pub struct Timer {
  elapsed: Duration,
}

impl Timer {
  pub fn new() -> Self {
    Self {
      elapsed: Duration::ZERO,
    }
  }

  // pub fn start(&mut self) {
  //   self.running = true;
  // }
  //
  // pub fn stop(&mut self) {
  //   self.running = false;
  // }
  //
  // pub fn reset(&mut self) {
  //   self.elapsed = Duration::ZERO;
  // }

  pub fn update(&mut self, dt: f32) {
    // log::debug!("dt value in timer: {:?}", dt);
    self.elapsed += Duration::from_secs_f32(dt);
  }

  pub fn elapsed(&self) -> Duration {
    // log::debug!("Secs value from elapsed: {:?}", self.elapsed().as_secs());
    self.elapsed
  }
}
