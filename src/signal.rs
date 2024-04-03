pub mod combinations;
pub mod core;
pub mod math;
pub mod midi;
pub mod modulation;
pub mod osc;

pub trait Signal: Send {
  fn sample(&mut self, t: f64) -> f64;
}
