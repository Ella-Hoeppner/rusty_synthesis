use std::ops::{Add, Mul};

use crate::{derive_signal_ops, Product, Signal, Sum};

const TAU: f64 = 2.0 * std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Sin;
derive_signal_ops!(Sin);
impl Signal for Sin {
  fn sample(&mut self, t: f64) -> f64 {
    (t * TAU).sin()
  }
}

#[derive(Debug, Clone)]
pub struct Saw;
derive_signal_ops!(Saw);
impl Signal for Saw {
  fn sample(&mut self, t: f64) -> f64 {
    (t % 1.) * 2. - 1.
  }
}

#[derive(Debug, Clone)]
pub struct Square;
derive_signal_ops!(Square);
impl Signal for Square {
  fn sample(&mut self, t: f64) -> f64 {
    if (t % 1.) > 0.5 {
      1.
    } else {
      -1.
    }
  }
}

pub fn tri(mut x: f64) -> f64 {
  x %= 1.;
  4. * x.min((0.5 - x).max(x - 1.))
}

#[derive(Debug, Clone)]
pub struct Tri;
derive_signal_ops!(Tri);
impl Signal for Tri {
  fn sample(&mut self, t: f64) -> f64 {
    tri(t)
  }
}
