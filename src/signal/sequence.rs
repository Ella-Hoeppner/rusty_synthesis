use crate::{derive_signal_ops, Signal};

use super::{core::Const, envelope::ADSR};

pub struct GateBeat {
  spacing: f64,
  offset: f64,
  last_index: usize,
}
derive_signal_ops!(GateBeat);

impl GateBeat {
  pub fn new(spacing: f64, offset: f64) -> GateBeat {
    GateBeat {
      spacing,
      offset,
      last_index: 0,
    }
  }
}

impl Signal for GateBeat {
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

pub struct GateSeq {
  period: f64,
  times: Vec<f64>,
  outer_index: usize,
  inner_index: usize,
}

impl GateSeq {
  pub fn new(period: f64, times: Vec<f64>) -> Self {
    Self {
      period,
      times,
      outer_index: 0,
      inner_index: 0,
    }
  }
}

impl Signal for GateSeq {
  fn sample(&mut self, t: f64) -> f64 {
    let outer_index = (t / self.period) as usize;
    if outer_index > self.outer_index {
      self.inner_index = 0;
      self.outer_index = outer_index;
    }
    let p = (t % self.period) / self.period;
    if self.inner_index < self.times.len() && p > self.times[self.inner_index] {
      self.inner_index += 1;
      1.
    } else {
      0.
    }
  }
}

pub struct ValueBearingSustainedGateSeq<V: Clone> {
  period: f64,
  times: Vec<(f64, f64, V)>,
  outer_index: usize,
  inner_index: usize,
}

impl<V: Clone> ValueBearingSustainedGateSeq<V> {
  pub fn new(period: f64, times: Vec<(f64, f64, V)>) -> Self {
    Self {
      period,
      times,
      outer_index: 0,
      inner_index: 0,
    }
  }
  pub fn sample(&mut self, t: f64) -> Option<V> {
    let outer_index = (t / self.period) as usize;
    if outer_index > self.outer_index {
      self.inner_index = 0;
      self.outer_index = outer_index;
    }
    let p = (t % self.period) / self.period;
    if self.inner_index < self.times.len() && p > self.times[self.inner_index].1
    {
      self.inner_index += 1;
    }
    if self.inner_index < self.times.len() && p > self.times[self.inner_index].0
    {
      Some(self.times[self.inner_index].2.clone())
    } else {
      None
    }
  }
}

pub struct SustainedGateSeq(ValueBearingSustainedGateSeq<f64>);

impl SustainedGateSeq {
  pub fn new(period: f64, times: Vec<(f64, f64)>) -> Self {
    Self(ValueBearingSustainedGateSeq::new(
      period,
      times
        .into_iter()
        .map(|(start, end)| (start, end, 1.))
        .collect(),
    ))
  }
}

impl Signal for SustainedGateSeq {
  fn sample(&mut self, t: f64) -> f64 {
    self.0.sample(t).unwrap_or(0.)
  }
}

pub struct ADSRNoteSeq<S: Signal> {
  sequence: ValueBearingSustainedGateSeq<f64>,
  envelope: ADSR<Const, Const, Const, Const, SustainedGateSeq>,
  carrier: S,
  last_frequency: f64,
}

impl<S: Signal> ADSRNoteSeq<S> {
  pub fn new(
    adsr: (f64, f64, f64, f64),
    period: f64,
    times: Vec<(f64, f64, f64)>,
    carrier: S,
  ) -> Self {
    Self {
      sequence: ValueBearingSustainedGateSeq::new(period, times.clone()),
      envelope: ADSR::new(
        Const(adsr.0),
        Const(adsr.1),
        Const(adsr.2),
        Const(adsr.3),
        SustainedGateSeq::new(
          period,
          times
            .iter()
            .copied()
            .map(|(start, end, _)| (start, end))
            .collect(),
        ),
      ),
      carrier,
      last_frequency: 1.,
    }
  }
}

impl<S: Signal> Signal for ADSRNoteSeq<S> {
  fn sample(&mut self, t: f64) -> f64 {
    let frequency = self.sequence.sample(t).unwrap_or(self.last_frequency);
    self.last_frequency = frequency;
    let amplitude = self.envelope.sample(t);
    amplitude * (self.carrier.sample(frequency * t))
  }
}
