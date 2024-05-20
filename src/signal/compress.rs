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

pub struct Tanh<S: Signal>(pub S);
derive_signal_ops!(Tanh<S: Signal>);
impl<S: Signal> Signal for Tanh<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.0.sample(t).tanh()
  }
}

pub struct Clamp<C: Signal, S: Signal>(pub C, pub S);
derive_signal_ops!(Clamp<C:Signal, S: Signal>);
impl<C: Signal, S: Signal> Signal for Clamp<C, S> {
  fn sample(&mut self, t: f64) -> f64 {
    let c = self.0.sample(t);
    self.1.sample(t).max(-c).min(c)
  }
}
