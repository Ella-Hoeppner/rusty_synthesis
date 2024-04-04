use std::collections::VecDeque;

use rand::Rng;

use crate::{derive_signal_ops, filter, MultiSum, Signal, Tuned};
use take_mut::take;

pub struct VoiceAllocator<E: Signal, V: Signal> {
  voices: Vec<(E, V)>,
  active_voices: VecDeque<usize>,
  max_active: usize,
}
impl<E: Signal, V: Signal> VoiceAllocator<E, V> {
  pub fn new(voices: Vec<(E, V)>, max_active: usize) -> Self {
    VoiceAllocator {
      voices,
      active_voices: VecDeque::new(),
      max_active,
    }
  }
}
impl<E: Signal, V: Signal> Signal for VoiceAllocator<E, V> {
  fn sample(&mut self, t: f64) -> f64 {
    let mut sum = 0.;
    let mut newly_activated_voices = vec![];
    take(&mut self.active_voices, |active_voices| {
      let envelope_values: Vec<_> = self
        .voices
        .iter_mut()
        .enumerate()
        .map(|(voice_index, (envelope, _))| {
          let envelope_value = envelope.sample(t);
          if envelope_value != 0. {
            if !active_voices.contains(&voice_index) {
              newly_activated_voices.push(voice_index);
            }
          }
          envelope_value
        })
        .collect();
      let mut filtered_active_voices: VecDeque<_> = active_voices
        .into_iter()
        .filter_map(|active_voice_index| {
          let (_, voice) = &mut self.voices[active_voice_index];
          let envelope_value = envelope_values[active_voice_index];
          if envelope_value != 0. {
            sum += envelope_value * voice.sample(t);
            Some(active_voice_index)
          } else {
            None
          }
        })
        .collect();
      for newly_activated_voice in newly_activated_voices {
        filtered_active_voices.push_back(newly_activated_voice);
      }
      while filtered_active_voices.len() > self.max_active {
        filtered_active_voices.pop_front();
      }
      filtered_active_voices
    });
    sum
  }
}

#[derive(Debug, Clone)]
pub struct DetunedSum<S: Signal + Clone> {
  sum: MultiSum<Tuned<S>>,
}
derive_signal_ops!(DetunedSum<S:Signal + Clone>);
impl<S: Signal + Clone> DetunedSum<S> {
  pub fn from_frequencies<T: Into<S> + Clone>(
    template: T,
    tunes: Vec<f64>,
  ) -> Self {
    DetunedSum {
      sum: MultiSum(
        tunes
          .iter()
          .map(|&tune| Tuned(tune, template.clone().into()))
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
