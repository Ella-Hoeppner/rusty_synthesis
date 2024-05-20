mod midi;
mod output;
mod signal;
mod synths;
mod util;

use midi::*;
use signal::{
  combinations::*, compress::*, core::*, envelope::*, filter::*, math::*,
  midi::*, modulation::*, osc::*, sequence::*, shaping::*, *,
};
use synths::*;
use util::*;

fn main() {
  let midi = MidiListener::start().unwrap();
  let full_polyphony_signal = Sigmoid(Sum(
    0.0
      * Product(
        Sigmoid(
          6. * Wavefold(
            8. * OnePoleLowPass::new(
              Modified(
                move |x| 0.001f64.powf(1. - x),
                MidiModWheel::new(&midi.ledger),
              ),
              10.
                >> DetunedSum::random_const(
                  (0.5 * (2. >> ((3. >> (0.25 * Tri)) >> Sin))) >> Saw,
                  16,
                  0.0035,
                ),
            ),
          ),
        ),
        AttackExpDecay::new(Const(0.001), Const(24.), Beat::new(0.5, 0.)),
      ),
    EnvelopedVoiceAllocator::new(
      8,
      (41..=72)
        .map(|note_index| {
          let note_frequency = ((note_index as f64 - 42.) / 12.).exp2();
          let envelope = Cached::new(
            ADSR::constant(
              0.05,
              0.05,
              0.25,
              0.2,
              MidiNote::new(note_index, &midi.ledger),
            ),
            /*AttackExpDecay::new(
              0.001,
              12.,
              MidiNote::new(note_index, &midi.ledger),
            ),*/
          );
          (
            envelope,
            meta_self_phase_mod_keys(note_frequency, &midi),
            //bwaaa_distorted_phase_mod(note_frequency, &midi),
            //distorted_fmod_keys(note_frequency, &midi),
          )
        })
        .collect(),
    ),
  ));
  output::begin(Box::new(full_polyphony_signal)).unwrap();
}
