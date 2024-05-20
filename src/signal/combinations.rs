use std::collections::VecDeque;

use rand::Rng;

use crate::{derive_signal_ops, Const, Signal};
use take_mut::take;

const ENVELOPE_THRESHOLD: f64 = 0.001;

pub struct Enveloped<E: Signal, S: Signal>(pub E, pub S);
derive_signal_ops!(Enveloped<E:Signal, S:Signal>);
impl<E: Signal, S: Signal> Signal for Enveloped<E, S> {
  fn sample(&mut self, t: f64) -> f64 {
    let envelope_value = self.0.sample(t);
    if envelope_value.abs() > ENVELOPE_THRESHOLD {
      envelope_value * self.1.sample(t)
    } else {
      0.
    }
  }
}

pub struct EnvelopedVoiceAllocator<E: Signal, V: Signal> {
  voices: Vec<(E, V)>,
  active_voices: VecDeque<usize>,
  max_active: usize,
  activation_threshold: f64,
}
derive_signal_ops!(EnvelopedVoiceAllocator<E:Signal, V:Signal>);
impl<E: Signal, V: Signal> EnvelopedVoiceAllocator<E, V> {
  pub fn new(max_active: usize, voices: Vec<(E, V)>) -> Self {
    EnvelopedVoiceAllocator {
      voices,
      active_voices: VecDeque::new(),
      max_active,
      activation_threshold: ENVELOPE_THRESHOLD,
    }
  }
  pub fn with_threshold(mut self, activation_threshold: f64) -> Self {
    self.activation_threshold = activation_threshold;
    self
  }
}
impl<E: Signal, V: Signal> Signal for EnvelopedVoiceAllocator<E, V> {
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
          if envelope_value.abs() > self.activation_threshold {
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
          if envelope_value.abs() > self.activation_threshold {
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
pub struct DetunedSum<F: Signal, S: Signal + Clone> {
  subvoices: Vec<(F, S)>,
}
derive_signal_ops!(DetunedSum<F:Signal, S:Signal + Clone>);
impl<F: Signal, S: Signal + Clone> DetunedSum<F, S> {
  pub fn from_frequencies(template: S, frequencies: Vec<F>) -> Self {
    DetunedSum {
      subvoices: frequencies
        .into_iter()
        .map(|frequency| (frequency, template.clone()))
        .collect(),
    }
  }
}
impl<S: Signal + Clone> DetunedSum<Const, S> {
  pub fn random_const(template: S, copies: u64, detune_factor: f64) -> Self {
    Self::from_frequencies(
      template,
      (0..copies)
        .map(|_| {
          Const(
            ((rand::thread_rng().gen::<f64>() * 2. - 1.) * detune_factor)
              .exp2(),
          )
        })
        .collect(),
    )
  }
  pub fn even_const(template: S, copies: u64, detune_factor: f64) -> Self {
    Self::from_frequencies(
      template,
      match copies {
        0 => vec![],
        1 => vec![Const(1.)],
        _ => (0..copies)
          .map(|copy_index| {
            Const(
              ((((copy_index as f64) / ((copies - 1) as f64)) * 2. - 1.)
                * detune_factor)
                .exp2(),
            )
          })
          .collect(),
      },
    )
  }
}
impl<F: Signal, S: Signal + Clone> Signal for DetunedSum<F, S> {
  fn sample(&mut self, t: f64) -> f64 {
    self
      .subvoices
      .iter_mut()
      .map(|(frequency, subvoice)| subvoice.sample(frequency.sample(t) * t))
      .sum()
  }
}
