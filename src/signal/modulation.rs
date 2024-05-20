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

#[derive(Debug, Clone)]
pub struct FreqMod<C: Signal, M: Signal> {
  carrier: C,
  modulator: M,
  last_t: f64,
  integrated_t: f64,
}
derive_signal_ops!(FreqMod<C: Signal, M: Signal>);
impl<C: Signal, M: Signal> FreqMod<C, M> {
  pub fn new(modulator: M, carrier: C) -> Self {
    Self {
      carrier,
      modulator,
      last_t: 0.,
      integrated_t: 0.,
    }
  }
}
impl<C: Signal, M: Signal> Signal for FreqMod<C, M> {
  fn sample(&mut self, t: f64) -> f64 {
    let dt = t - self.last_t;
    self.integrated_t += dt * self.modulator.sample(t);
    self.last_t = t;
    self.carrier.sample(self.integrated_t)
  }
}
