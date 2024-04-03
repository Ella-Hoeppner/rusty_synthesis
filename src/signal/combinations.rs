use rand::Rng;

use crate::{MultiSum, Signal, Tuned};

#[derive(Debug, Clone)]
pub struct DetunedSum<S: Signal + Clone> {
  sum: MultiSum<Tuned<S>>,
}

impl<S: Signal + Clone> DetunedSum<S> {
  pub fn from_frequencies<T: Into<S> + Clone>(
    template: T,
    tunes: Vec<f64>,
  ) -> Self {
    DetunedSum {
      sum: MultiSum::new(
        tunes
          .iter()
          .map(|&tune| Tuned::new(tune, template.clone().into()))
          .collect(),
      ),
    }
  }
  pub fn random<T: Into<S> + Clone>(
    template: T,
    copies: u64,
    detune_factor: f64,
  ) -> Self {
    Self::from_frequencies(
      template,
      (0..copies)
        .map(|_| {
          ((rand::thread_rng().gen::<f64>() * 2. - 1.) * detune_factor).exp2()
        })
        .collect(),
    )
  }
  pub fn even<T: Into<S> + Clone>(
    template: T,
    copies: u64,
    detune_factor: f64,
  ) -> Self {
    Self::from_frequencies(
      template,
      match copies {
        0 => vec![],
        1 => vec![1.],
        _ => (0..copies)
          .map(|copy_index| {
            ((((copy_index as f64) / ((copies - 1) as f64)) * 2. - 1.)
              * detune_factor)
              .exp2()
          })
          .collect(),
      },
    )
  }
}

impl<S: Signal + Clone> Signal for DetunedSum<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.sum.sample(t)
  }
}
