mod midi;
mod output;
mod signal;
mod util;

use midi::*;
use signal::{
  combinations::*, compress::*, core::*, envelope::*, filter::*, math::*,
  midi::*, modulation::*, osc::*, *,
};
use util::*;

fn main() {
  let midi = MidiListener::start().unwrap();
  let notes: Vec<_> = (41..=72)
    .map(|note_index| {
      let note_frequency = ((note_index - 41) as f64 / 12.).exp2();
      let envelope = Cached::new(ADSR::new(
        0.1,
        0.1,
        0.11,
        0.35,
        MidiNote::new(note_index, &midi.ledger),
      ));
      (
        envelope,
        //Tuned(440. * note_frequency, Sin),
        //Tuned(440. * note_frequency, Pure(|a| (a % 1.) * 2. - 1.)),
        /*DynamicOnePoleLowPass::new(
          Modified(|x| 0.1f64.powf(1. - x), MidiModWheel::new(&midi.ledger)),
          (90. * note_frequency)
            >> PhaseMod::with_self(
              PhaseMod::with_self(
                PhaseMod::with_self(
                  DetunedSum::random(Tri, 3, 0.025),
                  |s: DetunedSum<Tri>| 0.035 * (4. >> s),
                ),
                |s| 0.025 * (1.5 >> s),
              ),
              |s| 0.1 * (2. >> s),
            ),
        ),*/
        Const(0.)
          >> DynamicOnePoleLowPass::new(
            Modified(|x| 0.01f64.powf(1. - x), MidiModWheel::new(&midi.ledger)),
            (90. * note_frequency)
              >> DetunedSum::<
                PhaseMod<Saw, Tuned<PhaseMod<Sin, Tuned<Scaled<Tri>>>>>,
              >::random(
                (2. >> ((3. >> (0.75 * Tri)) >> Sin)) >> Saw, 8, 0.01
              ),
          ),
      )
    })
    .collect();
  let full_polyphony_signal = Sigmoid(
    VoiceAllocator::new(notes, 4),
    /*MultiSum(
      notes.into_iter().map(|(e, v)| Product(e, v)).collect(),
    )*/
  );
  output::begin(Box::new(full_polyphony_signal)).unwrap();
}
