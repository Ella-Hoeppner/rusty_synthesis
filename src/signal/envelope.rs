use crate::util::mix;
use crate::{derive_signal_ops, Signal};

pub enum ADSRState {
  Off,
  Attack(f64),
  Decay(f64),
  Sustain,
  Release(f64),
}

pub struct ADSR<S: Signal> {
  attack: f64,
  decay: f64,
  sustain: f64,
  release: f64,
  state: ADSRState,
  gate: S,
}
derive_signal_ops!(ADSR<S: Signal>);
use ADSRState::*;

impl<S: Signal> ADSR<S> {
  pub fn new(
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,
    gate: S,
  ) -> Self {
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

impl<S: Signal> Signal for ADSR<S> {
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
        let decay_start = start_t + self.attack;
        if t >= decay_start {
          Decay(decay_start)
        } else {
          Attack(start_t)
        }
      }
      Decay(start_t) => {
        let sustain_start = start_t + self.decay;
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
        let end_t = start_t + self.release;
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
      Attack(start_t) => (t - start_t) / self.attack,
      Decay(start_t) => mix(1., self.sustain, (t - start_t) / self.decay),
      Sustain => self.sustain,
      Release(start_t) => self.sustain * (1.0 - (t - start_t) / self.release),
    }
  }
}
