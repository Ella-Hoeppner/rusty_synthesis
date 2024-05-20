use crate::{derive_signal_ops, Signal};

pub struct KarplusStrong<G: Signal> {
  gate: G,
  delay_line: Vec<f64>,
  write_index: usize,
}
derive_signal_ops!(KarplusStrong<G:Signal>);
impl<G: Signal> KarplusStrong<G> {
  pub fn new(gate: G, delay_length: usize) -> Self {
    KarplusStrong {
      gate,
      delay_line: vec![0.0; delay_length],
      write_index: 0,
    }
  }
}
impl<G: Signal> Signal for KarplusStrong<G> {
  fn sample(&mut self, t: f64) -> f64 {
    if self.gate.sample(t) > 0.5 {
      self.delay_line[self.write_index] =
        ((t * 6123.712).sin() * 1238.2).rem_euclid(1.0) * 2. - 1.;
    }
    let value = self.delay_line[self.write_index];
    let next_value = (value
      + self.delay_line[(self.write_index + 1) % self.delay_line.len()])
      / 2.0;
    self.delay_line[self.write_index] = next_value;
    self.write_index = (self.write_index + 1) % self.delay_line.len();
    next_value
  }
}
