const TAU: f64 = 2.0 * std::f64::consts::PI;

const DEFAULT_WAVETABLE_RESOLUTION: usize = 10000;

pub trait Signal: Send {
  fn sample(&mut self, t: f64) -> f64;
}

#[derive(Debug, Clone)]
pub struct Cos {}

impl Cos {
  pub fn new() -> Self {
    Cos {}
  }
}

impl Signal for Cos {
  fn sample(&mut self, t: f64) -> f64 {
    (t * TAU).cos()
  }
}

#[derive(Debug, Clone)]
pub struct Saw {}
impl Saw {
  pub fn new() -> Self {
    Saw {}
  }
}

impl Signal for Saw {
  fn sample(&mut self, t: f64) -> f64 {
    (t % 1.) * 2. - 1.
  }
}

pub struct WithFrequency<S: Signal> {
  child: S,
  freq: f64,
}

impl<S: Signal> WithFrequency<S> {
  pub fn new(child: S, freq: f64) -> Self {
    WithFrequency::<S> { freq, child }
  }
}

impl<S: Signal> Signal for WithFrequency<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.child.sample(t * self.freq)
  }
}

pub enum WavetableSampleStrategy {
  Linear,
  Quadratic,
}

pub struct Wavetabled<S: Signal> {
  child: S,
  table: Vec<f64>,
  strategy: WavetableSampleStrategy,
}

impl<S: Signal> Wavetabled<S> {
  pub fn new_with(
    mut child: S,
    resolution: usize,
    strategy: WavetableSampleStrategy,
  ) -> Self {
    let table = (0..resolution)
      .map(|i| child.sample((i as f64) / (resolution as f64)))
      .collect();
    Wavetabled::<S> {
      child,
      table,
      strategy,
    }
  }
  pub fn new(child: S) -> Self {
    Self::new_with(
      child,
      DEFAULT_WAVETABLE_RESOLUTION,
      WavetableSampleStrategy::Quadratic,
    )
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

pub struct Product<S1: Signal, S2: Signal> {
  child_1: S1,
  child_2: S2,
}

impl<S1: Signal, S2: Signal> Product<S1, S2> {
  pub fn new(child_1: S1, child_2: S2) -> Self {
    Product::<S1, S2> { child_1, child_2 }
  }
}

impl<S1: Signal, S2: Signal> Signal for Product<S1, S2> {
  fn sample(&mut self, t: f64) -> f64 {
    self.child_1.sample(t) * self.child_2.sample(t)
  }
}
