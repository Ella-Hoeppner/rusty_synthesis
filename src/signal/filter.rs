use crate::Signal;

pub struct OnePoleLowPass<S: Signal> {
  pub a0: f64,
  pub last_value: f64,
  pub child: S,
}

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
