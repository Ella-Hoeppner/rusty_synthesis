mod midi;
mod output;
mod signal;

use midi::*;
use signal::{
  combinations::*, core::*, math::*, midi::*, modulation::*, osc::*, *,
};

fn main() {
  let midi_listener = MidiListener::start().unwrap();
  let full_polyphony_signal = Tuned::new(
    1.,
    MultiSum::new(
      (41..=51)
        .map(|note_index| {
          let note_frequency = ((note_index - 41) as f64 / 12.).exp2();
          Product::new(
            MidiNoteSignal::new(note_index, midi_listener.get_ledger()),
            Tuned::new(
              440. * note_frequency,
              PhaseMod::with_self(
                PhaseMod::with_self(
                  DetunedClones::even(Sin {}, 3, 0.035),
                  |s| Scaled::new(0.025, Tuned::new(2., s)),
                ),
                |s| Scaled::new(0.0125, Tuned::new(5., s)),
              ),
            ),
          )
        })
        .collect(),
    ),
  );
  output::begin(Box::new(full_polyphony_signal)).unwrap();
}
