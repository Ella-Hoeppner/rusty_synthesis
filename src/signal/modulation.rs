use crate::Signal;

#[derive(Debug, Clone)]
pub struct PhaseMod<C: Signal, M: Signal> {
  carrier: C,
  modulator: M,
}

impl<C: Signal, M: Signal> PhaseMod<C, M> {
  pub fn new(carrier: C, modulator: M) -> Self {
    PhaseMod { carrier, modulator }
  }
}

impl<C: Signal + Clone, M: Signal> PhaseMod<C, M> {
  pub fn with_self<F: Fn(C) -> M>(carrier: C, f: F) -> Self {
    Self {
      carrier: carrier.clone(),
      modulator: f(carrier),
    }
  }
}

impl<C: Signal, M: Signal> Signal for PhaseMod<C, M> {
  fn sample(&mut self, t: f64) -> f64 {
    self.carrier.sample(t + self.modulator.sample(t))
  }
}
