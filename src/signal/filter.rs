use crate::{derive_signal_ops, Signal};

pub struct OnePoleLowPass<S: Signal> {
  pub a0: f64,
  pub last_value: f64,
  pub child: S,
}
derive_signal_ops!(OnePoleLowPass<S: Signal>);
impl<S: Signal> OnePoleLowPass<S> {
  pub fn new(a0: f64, child: S) -> Self {
    OnePoleLowPass {
      a0,
      last_value: 0.,
      child,
    }
  }
}

impl<S: Signal> Signal for OnePoleLowPass<S> {
  fn sample(&mut self, t: f64) -> f64 {
    let value =
      self.a0 * self.child.sample(t) + (1. - self.a0) * self.last_value;
    self.last_value = value;
    value
  }
}

pub struct DynamicOnePoleLowPass<C: Signal, S: Signal> {
  pub cutoff: C,
  pub last_value: f64,
  pub child: S,
}
derive_signal_ops!(DynamicOnePoleLowPass<C:Signal, S: Signal>);
impl<C: Signal, S: Signal> DynamicOnePoleLowPass<C, S> {
  pub fn new(cutoff: C, child: S) -> Self {
    DynamicOnePoleLowPass {
      cutoff,
      last_value: 0.,
      child,
    }
  }
}

impl<C: Signal, S: Signal> Signal for DynamicOnePoleLowPass<C, S> {
  fn sample(&mut self, t: f64) -> f64 {
    let a0 = self.cutoff.sample(t);
    let value = a0 * self.child.sample(t) + (1. - a0) * self.last_value;
    self.last_value = value;
    value
  }
}
