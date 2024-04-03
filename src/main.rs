mod midi;
mod output;
mod signal;

use midi::*;
use signal::{
  combinations::*, core::*, envelope::*, filter::*, math::*, midi::*,
  modulation::*, osc::*, *,
};

fn main() {
  let midi_listener = MidiListener::start().unwrap();
  let full_polyphony_signal = Tuned::new(
    1.,
    MultiSum::new(
      (41..=52)
        .map(|note_index| {
          let note_frequency = ((note_index - 41) as f64 / 12.).exp2();
          Product::new(
            ADSR::new(
              0.1,
              0.1,
              0.11,
              0.35,
              MidiNoteSignal::new(note_index, midi_listener.get_ledger()),
            ),
            //Tuned::new(440. * note_frequency, Sin),
            OnePoleLowPass::new(
              0.25,
              Tuned::new(
                90. * note_frequency,
                PhaseMod::with_self(
                  PhaseMod::with_self(
                    PhaseMod::with_self(
                      DetunedSum::random(Tri, 3, 0.025),
                      |s: DetunedSum<Tri>| {
                        Scaled::new(0.035, Tuned::new(4., s))
                      },
                    ),
                    |s| Scaled::new(0.025, Tuned::new(5., s)),
                  ),
                  |s| Scaled::new(0.05, Tuned::new(7., s)),
                ),
              ),
            ),
          )
        })
        .collect(),
    ),
  );
  output::begin(Box::new(full_polyphony_signal)).unwrap();
}
