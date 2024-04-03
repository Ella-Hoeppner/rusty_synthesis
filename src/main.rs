mod midi;
mod output;
mod signal;

use midi::*;
use signal::{core::*, math::*, midi::*, osc::*, *};

fn main() {
  let midi_listener = MidiListener::start().unwrap();
  let full_polyphony_signal = MultiSum::new(
    (41..=72)
      .map(|note_index| {
        let note_frequency = 110. * ((note_index - 41) as f64 / 12.).exp2();
        Product::new(
          MidiNoteSignal::new(note_index, midi_listener.get_ledger()),
          MultiProduct::new(
            [0.5, 1.]
              .into_iter()
              .map(|frequency_multiple| {
                WithFrequency::new(
                  Saw::new(),
                  note_frequency * frequency_multiple,
                )
              })
              .collect(),
          ),
        )
      })
      .collect(),
  );
  output::begin(Box::new(full_polyphony_signal)).unwrap();
}
