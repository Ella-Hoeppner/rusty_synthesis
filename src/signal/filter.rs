use crate::{derive_signal_ops, Signal};

pub struct OnePoleLowPass<A: Signal, S: Signal> {
  pub a0: A,
  pub last_value: f64,
  pub child: S,
}
derive_signal_ops!(OnePoleLowPass<A:Signal, S: Signal>);
impl<A: Signal, S: Signal> OnePoleLowPass<A, S> {
  pub fn new(a0: A, child: S) -> Self {
    OnePoleLowPass {
      a0,
      last_value: 0.,
      child,
    }
  }
}

impl<A: Signal, S: Signal> Signal for OnePoleLowPass<A, S> {
  fn sample(&mut self, t: f64) -> f64 {
    let a0 = self.a0.sample(t);
    let value = a0 * self.child.sample(t) + (1. - a0) * self.last_value;
    self.last_value = value;
    value
  }
}
