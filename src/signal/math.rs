use crate::{derive_signal_ops, Signal};

#[derive(Debug, Clone)]
pub struct Product<S1: Signal, S2: Signal>(pub S1, pub S2);
derive_signal_ops!(Product<S1:Signal, S2: Signal>);
impl<S1: Signal, S2: Signal> Signal for Product<S1, S2> {
  fn sample(&mut self, t: f64) -> f64 {
    self.0.sample(t) * self.1.sample(t)
  }
}

#[derive(Debug, Clone)]
pub struct MultiProduct<S: Signal>(pub Vec<S>);
derive_signal_ops!(MultiProduct<S:Signal>);
impl<S: Signal> Signal for MultiProduct<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.0.iter_mut().map(|child| child.sample(t)).product()
  }
}

#[derive(Debug, Clone)]
pub struct Sum<S1: Signal, S2: Signal>(pub S1, pub S2);
derive_signal_ops!(Sum<S1:Signal, S2: Signal>);
impl<S1: Signal, S2: Signal> Signal for Sum<S1, S2> {
  fn sample(&mut self, t: f64) -> f64 {
    self.0.sample(t) + self.1.sample(t)
  }
}

#[derive(Debug, Clone)]
pub struct MultiSum<S: Signal>(pub Vec<S>);
derive_signal_ops!(MultiSum<S:Signal>);
impl<S: Signal> Signal for MultiSum<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.0.iter_mut().map(|child| child.sample(t)).sum()
  }
}

#[derive(Debug, Clone)]
pub struct Pow<S1: Signal, S2: Signal>(pub S1, pub S2);
derive_signal_ops!(Pow<S1:Signal, S2: Signal>);
impl<S1: Signal, S2: Signal> Signal for Pow<S1, S2> {
  fn sample(&mut self, t: f64) -> f64 {
    let base = self.0.sample(t);
    base.signum() * base.abs().powf(self.1.sample(t))
  }
}
