use crate::Signal;

const DEFAULT_WAVETABLE_RESOLUTION: usize = 10000;

#[derive(Debug, Clone)]
pub struct Nil {}

impl Nil {
  pub fn new() -> Self {
    Nil {}
  }
}

impl Signal for Nil {
  fn sample(&mut self, _t: f64) -> f64 {
    0.
  }
}

#[derive(Debug, Clone)]
pub struct Tuned<S: Signal> {
  freq: f64,
  child: S,
}

impl<S: Signal> Tuned<S> {
  pub fn new(freq: f64, child: S) -> Self {
    Tuned::<S> { freq, child }
  }
}

impl<S: Signal> Signal for Tuned<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.child.sample(t * self.freq)
  }
}

#[derive(Debug, Clone)]
pub struct Scaled<S: Signal> {
  scale: f64,
  child: S,
}

impl<S: Signal> Scaled<S> {
  pub fn new(scale: f64, child: S) -> Self {
    Scaled::<S> { scale, child }
  }
}

impl<S: Signal> Signal for Scaled<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.scale * self.child.sample(t)
  }
}

#[derive(Debug, Clone)]
pub enum WavetableSampleStrategy {
  Linear,
  Quadratic,
}

#[derive(Debug, Clone)]
pub struct Wavetabled<S: Signal> {
  child: S,
  table: Vec<f64>,
  strategy: WavetableSampleStrategy,
}

impl<S: Signal> Wavetabled<S> {
  pub fn new(resolution: usize, mut child: S) -> Self {
    let table = (0..resolution)
      .map(|i| child.sample((i as f64) / (resolution as f64)))
      .collect();
    Wavetabled::<S> {
      child,
      table,
      strategy: WavetableSampleStrategy::Quadratic,
    }
  }
  pub fn with_strategy(mut self, strategy: WavetableSampleStrategy) -> Self {
    self.strategy = strategy;
    self
  }
}

impl<S: Signal> Signal for Wavetabled<S> {
  fn sample(&mut self, t: f64) -> f64 {
    let t = t % 1.;
    let i = self.table.len() as f64 * t;
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
pub struct ToUni<S: Signal> {
  child: S,
}

impl<S: Signal> ToUni<S> {
  pub fn new(child: S) -> Self {
    ToUni::<S> { child }
  }
}

impl<S: Signal> Signal for ToUni<S> {
  fn sample(&mut self, t: f64) -> f64 {
    (self.child.sample(t) + 1.) * 0.5
  }
}

#[derive(Debug, Clone)]
pub struct FromUni<S: Signal> {
  child: S,
}

impl<S: Signal> FromUni<S> {
  pub fn new(child: S) -> Self {
    FromUni::<S> { child }
  }
}

impl<S: Signal> Signal for FromUni<S> {
  fn sample(&mut self, t: f64) -> f64 {
    (self.child.sample(t) * 2.) - 1.
  }
}
