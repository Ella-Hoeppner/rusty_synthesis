use std::sync::{Arc, Mutex};

pub mod combinations;
pub mod compress;
pub mod core;
pub mod envelope;
pub mod filter;
pub mod math;
pub mod midi;
pub mod modulation;
pub mod osc;
pub mod sequence;
pub mod shaping;
pub mod waveguide;

pub trait Signal: Send {
  fn sample(&mut self, t: f64) -> f64;
}

pub struct Pure<F: FnMut(f64) -> f64>(pub F);
//derive_signal_ops!(Pure<F: FnMut(f64) -> f64>);
impl<F: FnMut(f64) -> f64 + std::marker::Send> Signal for Pure<F> {
  fn sample(&mut self, t: f64) -> f64 {
    (self.0)(t)
  }
}

impl<F: FnMut(f64) -> f64> From<F> for Pure<F> {
  fn from(f: F) -> Self {
    Self(f)
  }
}

pub struct Modified<F: Fn(f64) -> f64, S: Signal>(pub F, pub S);
//derive_signal_ops!(Modified<F: FnMut(f64) -> f64>, S:Signal);
impl<F: Fn(f64) -> f64 + Send, S: Signal> Signal for Modified<F, S> {
  fn sample(&mut self, t: f64) -> f64 {
    (self.0)(self.1.sample(t))
  }
}

pub struct Composed<S1: Signal, S2: Signal>(pub S1, pub S2);
derive_signal_ops!(Composed<S1: Signal, S2:Signal>);
impl<S1: Signal, S2: Signal> Signal for Composed<S1, S2> {
  fn sample(&mut self, t: f64) -> f64 {
    self.0.sample(self.1.sample(t))
  }
}

pub struct Cached<S: Signal> {
  signal_and_state: Arc<Mutex<(S, (f64, f64))>>,
}
//derive_signal_ops!(Cached<S: Signal>);
impl<S: Signal> Cached<S> {
  pub fn new(signal: S) -> Self {
    Self {
      signal_and_state: Arc::new((signal, (0.0, 0.0)).into()),
    }
  }
}
impl<S: Signal + Send + Sync> Signal for Cached<S> {
  fn sample(&mut self, t: f64) -> f64 {
    let mut x = self.signal_and_state.lock().unwrap();
    let (last_t, last_value) = x.1;
    if t == last_t {
      last_value
    } else {
      let value = x.0.sample(t);
      (*x).1 = (last_t, last_value);
      value
    }
  }
}

macro_rules! derive_signal_ops {
  (
    $t:ident
    $(<
        $( $g:ident $(: $b:ident $( + $bn:ident )* )? ),+
    >)?
  ) => {
    impl<_S_: Signal $($(, $g $(: $b $( + $bn)* )? )*)?> std::ops::Mul<_S_> for $t<$($($g,)*)?> {
      type Output = crate::signal::math::Product<Self, _S_>;
      fn mul(self, rhs: _S_) -> Self::Output {
        crate::signal::math::Product(self, rhs)
      }
    }
    impl $( < $($g $(: $b $( + $bn)* )?, )* > )? std::ops::Mul<f64> for $t<$($($g,)*)?> {
      type Output = crate::signal::core::Scaled<Self>;
      fn mul(self, rhs: f64) -> Self::Output {
        crate::signal::core::Scaled(rhs, self)
      }
    }
    impl $( < $($g $(: $b $( + $bn)* )?, )* > )? std::ops::Mul<$t<$($($g,)*)?>> for f64 {
      type Output = crate::signal::core::Scaled<$t<$($($g,)*)?>>;
      fn mul(self, rhs: $t<$($($g,)*)?>) -> Self::Output {
        crate::signal::core::Scaled(self, rhs)
      }
    }
    impl<_S_: Signal $($(, $g $(: $b $( + $bn)* )? )*)?> std::ops::Add<_S_> for $t<$($($g,)*)?> {
      type Output = crate::signal::math::Sum<Self, _S_>;
      fn add(self, rhs: _S_) -> Self::Output {
        crate::signal::math::Sum(self, rhs)
      }
    }
    impl<_S_: Signal $($(, $g $(: $b $( + $bn)* )? )*)?> std::ops::Shr<_S_> for $t<$($($g,)*)?> {
      type Output = crate::signal::modulation::PhaseMod<_S_, Self>;
      fn shr(self, rhs: _S_) -> Self::Output {
        crate::signal::modulation::PhaseMod(self, rhs)
      }
    }
    impl $( < $($g $(: $b $( + $bn)* )?, )* > )? std::ops::Shr<$t<$($($g,)*)?>> for f64 {
      type Output = crate::signal::core::Tuned<$t<$($($g,)*)?>>;
      fn shr(self, rhs: $t<$($($g,)*)?>) -> Self::Output {
        crate::signal::core::Tuned(self, rhs)
      }
    }
  };
}
pub(crate) use derive_signal_ops;
