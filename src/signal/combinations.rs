use crate::{MultiSum, Signal, Tuned};

#[derive(Debug, Clone)]
pub struct DetunedClones<S: Signal + Clone> {
  sum: MultiSum<Tuned<S>>,
}

impl<S: Signal + Clone> DetunedClones<S> {
  pub fn from_frequencies(template: S, tunes: Vec<f64>) -> Self {
    DetunedClones {
      sum: MultiSum::new(
        tunes
          .iter()
          .map(|&tune| Tuned::new(tune, template.clone()))
          .collect(),
      ),
    }
  }
  pub fn even(template: S, copies: u64, detune_factor: f64) -> Self {
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

impl<S: Signal + Clone> Signal for DetunedClones<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.sum.sample(t)
  }
}
