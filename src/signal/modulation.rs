use crate::{derive_signal_ops, Signal};

#[derive(Debug, Clone)]
pub struct PhaseMod<C: Signal, M: Signal>(pub M, pub C);
derive_signal_ops!(PhaseMod<C: Signal, M: Signal>);
impl<C: Signal + Clone, M: Signal> PhaseMod<C, M> {
  pub fn with_self<F: Fn(C) -> M>(carrier: C, f: F) -> Self {
    Self(f(carrier.clone()), carrier)
  }
}
impl<C: Signal, M: Signal> Signal for PhaseMod<C, M> {
  fn sample(&mut self, t: f64) -> f64 {
    self.1.sample(t + self.0.sample(t))
  }
}
