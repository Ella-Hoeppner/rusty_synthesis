use crate::{signal::*, *};

pub fn meta_self_phase_mod_keys(
  frequency: f64,
  midi_listener: &MidiListener,
) -> impl Signal {
  OnePoleLowPass::new(
    Modified(
      |x| 0.1f64.powf(1. - x),
      MidiModWheel::new(&midi_listener.ledger),
    ),
    (90. * frequency)
      >> PhaseMod::with_self(
        PhaseMod::with_self(
          PhaseMod::with_self(
            DetunedSum::random_const(Tri, 3, 0.025),
            |s: DetunedSum<Const, Tri>| 0.035 * (4. >> s),
          ),
          |s| 0.025 * (1.5 >> s),
        ),
        |s| 0.1 * (2. >> s),
      ),
  )
}

pub fn bwaaa_distorted_phase_mod(
  frequency: f64,
  midi_listener: &MidiListener,
) -> impl Signal {
  Sigmoid(
    6. * Wavefold(
      8. * OnePoleLowPass::new(
        Modified(
          move |x| frequency * 0.001f64.powf(1. - x),
          MidiModWheel::new(&midi_listener.ledger),
        ),
        (30. * frequency)
          >> DetunedSum::random_const(
            (0.5 * (2. >> ((3. >> (0.25 * Tri)) >> Sin))) >> Saw,
            16,
            0.0035,
          ),
      ),
    ),
  )
}

pub fn distorted_fmod_keys(
  frequency: f64,
  midi_listener: &MidiListener,
) -> impl Signal {
  Sigmoid(
    8. * OnePoleLowPass::new(
      Modified(
        move |x| frequency * 0.001f64.powf(1. - x),
        MidiModWheel::new(&midi_listener.ledger),
      ),
      DetunedSum::even_const(
        FreqMod::new(
          Pow(Const(3.), (304. * frequency) >> Saw),
          FreqMod::new(
            Pow(Const(2.5), (578. * frequency) >> Saw),
            FreqMod::new(
              Pow(Const(2.75), (39. * frequency) >> Saw),
              (100. * frequency) >> Saw,
            ),
          ),
        ),
        8,
        0.0075,
      ),
    ),
  )
}
