use crate::{derive_signal_ops, Signal};

pub fn sigmoid(x: f64) -> f64 {
  1.0 / (1.0 + (-x).exp())
}

pub struct Sigmoid<S: Signal>(pub S);
derive_signal_ops!(Sigmoid<S: Signal>);
impl<S: Signal> Signal for Sigmoid<S> {
  fn sample(&mut self, t: f64) -> f64 {
    sigmoid(self.0.sample(t)) * 2. - 1.
  }
}
