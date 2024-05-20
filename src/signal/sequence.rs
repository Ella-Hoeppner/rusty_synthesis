use crate::{derive_signal_ops, Signal};

pub struct Beat {
  spacing: f64,
  offset: f64,
  last_index: usize,
}
derive_signal_ops!(Beat);

impl Beat {
  pub fn new(spacing: f64, offset: f64) -> Beat {
    Beat {
      spacing,
      offset,
      last_index: 0,
    }
  }
}

impl Signal for Beat {
  fn sample(&mut self, t: f64) -> f64 {
    let index = ((t - self.offset) / self.spacing) as usize;
    if index > self.last_index {
      self.last_index = index;
      1.
    } else {
      0.
    }
  }
}
