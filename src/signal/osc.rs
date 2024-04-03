use crate::Signal;

const TAU: f64 = 2.0 * std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Sin {}

impl Sin {
  pub fn new() -> Self {
    Sin {}
  }
}

impl Signal for Sin {
  fn sample(&mut self, t: f64) -> f64 {
    (t * TAU).sin()
  }
}

#[derive(Debug, Clone)]
pub struct Saw {}
impl Saw {
  pub fn new() -> Self {
    Saw {}
  }
}

impl Signal for Saw {
  fn sample(&mut self, t: f64) -> f64 {
    (t % 1.) * 2. - 1.
  }
}
