use crate::{derive_signal_ops, Signal};

#[derive(Debug, Clone)]
pub struct Const(pub f64);
derive_signal_ops!(Const);
impl Signal for Const {
  fn sample(&mut self, _t: f64) -> f64 {
    self.0
  }
}
impl From<f64> for Const {
  fn from(x: f64) -> Self {
    Const(x)
  }
}

#[derive(Debug, Clone)]
pub struct Tuned<S: Signal>(pub f64, pub S);
derive_signal_ops!(Tuned<S: Signal>);
impl<S: Signal> Signal for Tuned<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.1.sample(t * self.0)
  }
}

#[derive(Debug, Clone)]
pub struct Scaled<S: Signal>(pub f64, pub S);
derive_signal_ops!(Scaled<S: Signal>);
impl<S: Signal> Signal for Scaled<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.0 * self.1.sample(t)
  }
}

#[derive(Debug, Clone)]
pub enum WavetableSampleStrategy {
  Linear,
  Quadratic,
}

#[derive(Debug, Clone)]
pub struct Table<S: Signal> {
  child: S,
  length: f64,
  table: Vec<f64>,
  strategy: WavetableSampleStrategy,
}
derive_signal_ops!(Table<S: Signal>);

impl<S: Signal> Table<S> {
  pub fn new(length: f64, resolution: usize, mut child: S) -> Self {
    let table = (0..resolution)
      .map(|i| child.sample(length * (i as f64) / (resolution as f64)))
      .collect();
    Table::<S> {
      child,
      length,
      table,
      strategy: WavetableSampleStrategy::Quadratic,
    }
  }
  pub fn with_strategy(mut self, strategy: WavetableSampleStrategy) -> Self {
    self.strategy = strategy;
    self
  }
}

impl<S: Signal> Signal for Table<S> {
  fn sample(&mut self, t: f64) -> f64 {
    let x = (t / self.length) % 1.;
    let i = self.table.len() as f64 * x;
    let i_floored = i.floor() as usize;
    let p = i % 1.;
    match self.strategy {
      WavetableSampleStrategy::Linear => {
        let sample_1 = self.table[i_floored];
        let sample_2 = self.table[(i_floored + 1).rem_euclid(self.table.len())];
        sample_1 * (1. - p) + sample_2 * p
      }
      WavetableSampleStrategy::Quadratic => {
        let sample_1 = self.table
          [(i_floored as i64 - 1).rem_euclid(self.table.len() as i64) as usize];
        let sample_2 = self.table[i_floored];
        let sample_3 = self.table[(i_floored + 1).rem_euclid(self.table.len())];

        sample_2
          + (0.5
            * p
            * ((sample_3 - sample_1)
              + (sample_1 - 2. * sample_2 + sample_3) * p))
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct ToUni<S: Signal>(pub S);
derive_signal_ops!(ToUni<S: Signal>);
impl<S: Signal> Signal for ToUni<S> {
  fn sample(&mut self, t: f64) -> f64 {
    (self.0.sample(t) + 1.) * 0.5
  }
}

#[derive(Debug, Clone)]
pub struct FromUni<S: Signal>(pub S);
derive_signal_ops!(FromUni<S: Signal>);
impl<S: Signal> Signal for FromUni<S> {
  fn sample(&mut self, t: f64) -> f64 {
    (self.0.sample(t) * 2.) - 1.
  }
}
