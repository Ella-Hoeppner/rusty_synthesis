use crate::{derive_signal_ops, tri, Signal};

pub struct Wavefold<S: Signal>(pub S);
derive_signal_ops!(Wavefold<S: Signal>);
impl<S: Signal> Signal for Wavefold<S> {
  fn sample(&mut self, t: f64) -> f64 {
    tri(0.25 * self.0.sample(t))
  }
}
