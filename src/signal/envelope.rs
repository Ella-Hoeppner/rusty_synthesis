use crate::util::mix;
use crate::{derive_signal_ops, Const, Signal};

pub enum ADSRState {
  Off,
  Attack(f64),
  Decay(f64),
  Sustain,
  Release(f64),
}

pub struct ADSR<A: Signal, D: Signal, S: Signal, R: Signal, G: Signal> {
  attack: A,
  decay: D,
  sustain: S,
  release: R,
  state: ADSRState,
  gate: G,
}
derive_signal_ops!(ADSR<A:Signal, D:Signal, S:Signal, R:Signal, G: Signal>);
use ADSRState::*;

impl<A: Signal, D: Signal, S: Signal, R: Signal, G: Signal>
  ADSR<A, D, S, R, G>
{
  pub fn new(attack: A, decay: D, sustain: S, release: R, gate: G) -> Self {
    Self {
      attack,
      decay,
      sustain,
      release,
      state: ADSRState::Off,
      gate,
    }
  }
}

impl<G: Signal> ADSR<Const, Const, Const, Const, G> {
  pub fn constant(
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,
    gate: G,
  ) -> Self {
    Self::new(
      Const(attack),
      Const(decay),
      Const(sustain),
      Const(release),
      gate,
    )
  }
}

impl<A: Signal, D: Signal, S: Signal, R: Signal, G: Signal> Signal
  for ADSR<A, D, S, R, G>
{
  fn sample(&mut self, t: f64) -> f64 {
    let on = self.gate.sample(t) > 0.5;
    let new_state = match self.state {
      Off => {
        if on {
          Attack(t)
        } else {
          Off
        }
      }
      Attack(start_t) => {
        let decay_start = start_t + self.attack.sample(t);
        if t >= decay_start {
          Decay(decay_start)
        } else {
          Attack(start_t)
        }
      }
      Decay(start_t) => {
        let sustain_start = start_t + self.decay.sample(t);
        if t >= sustain_start {
          if on {
            Sustain
          } else {
            Release(sustain_start)
          }
        } else {
          Decay(start_t)
        }
      }
      Sustain => {
        if on {
          Sustain
        } else {
          Release(t)
        }
      }
      Release(start_t) => {
        let end_t = start_t + self.release.sample(t);
        if t >= end_t {
          Off
        } else {
          Release(start_t)
        }
      }
    };
    self.state = new_state;
    match self.state {
      Off => 0.0,
      Attack(start_t) => (t - start_t) / self.attack.sample(t),
      Decay(start_t) => mix(
        1.,
        self.sustain.sample(t),
        (t - start_t) / self.decay.sample(t),
      ),
      Sustain => self.sustain.sample(t),
      Release(start_t) => {
        self.sustain.sample(t) * (1.0 - (t - start_t) / self.release.sample(t))
      }
    }
  }
}

pub struct ExpImpulse<S: Signal> {
  gate: S,
  active: bool,
  start_t: f64,
}
derive_signal_ops!(ExpImpulse<S: Signal>);
impl<S: Signal> ExpImpulse<S> {
  pub fn new(gate: S) -> Self {
    Self {
      gate,
      active: false,
      start_t: -f64::MAX,
    }
  }
}
impl<S: Signal> Signal for ExpImpulse<S> {
  fn sample(&mut self, t: f64) -> f64 {
    if self.gate.sample(t) > 0.5 {
      if !self.active {
        self.active = true;
        self.start_t = t;
      }
    } else {
      if self.active {
        self.active = false;
      }
    }
    let adjusted_t = t - self.start_t;
    adjusted_t * (1. - adjusted_t).exp()
  }
}

pub struct ExpDecay<S: Signal> {
  gate: S,
  active: bool,
  start_t: f64,
}
derive_signal_ops!(ExpDecay<S: Signal>);
impl<S: Signal> ExpDecay<S> {
  pub fn new(gate: S) -> Self {
    Self {
      gate,
      active: false,
      start_t: -f64::MAX,
    }
  }
}
impl<S: Signal> Signal for ExpDecay<S> {
  fn sample(&mut self, t: f64) -> f64 {
    if self.gate.sample(t) > 0.5 {
      if !self.active {
        self.active = true;
        self.start_t = t;
        println!("activating!!");
      }
    } else {
      if self.active {
        println!("deactivating!!");
        self.active = false;
      }
    }
    let adjusted_t = t - self.start_t;
    1. / adjusted_t.exp()
  }
}

pub struct AttackExpDecay<A: Signal, D: Signal, S: Signal> {
  gate: S,
  active: bool,
  start_t: f64,
  attack: A,
  decay_factor: D,
}
derive_signal_ops!(AttackExpDecay<A:Signal, D:Signal, S: Signal>);
impl<A: Signal, D: Signal, S: Signal> AttackExpDecay<A, D, S> {
  pub fn new(attack: A, decay_factor: D, gate: S) -> Self {
    Self {
      gate,
      attack,
      decay_factor,
      active: false,
      start_t: -f64::MAX,
    }
  }
}
impl<S: Signal> AttackExpDecay<Const, Const, S> {
  pub fn constant(attack: f64, decay_factor: f64, gate: S) -> Self {
    Self::new(Const(attack), Const(decay_factor), gate)
  }
}
impl<A: Signal, D: Signal, S: Signal> Signal for AttackExpDecay<A, D, S> {
  fn sample(&mut self, t: f64) -> f64 {
    if self.gate.sample(t) > 0.5 {
      if !self.active {
        self.active = true;
        self.start_t = t;
      }
    } else {
      if self.active {
        self.active = false;
      }
    }
    let adjusted_t = t - self.start_t;
    let attack = self.attack.sample(t);
    if adjusted_t < attack {
      adjusted_t / attack
    } else {
      1. / (self.decay_factor.sample(t) * adjusted_t).exp()
    }
  }
}
