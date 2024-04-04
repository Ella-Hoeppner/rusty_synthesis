use std::sync::Arc;
use std::sync::Mutex;

use crate::derive_signal_ops;
use crate::Signal;

use crate::MidiLedger;

#[derive(Debug, Clone)]
pub struct MidiNote {
  note: u8,
  ledger: Arc<Mutex<MidiLedger>>,
}
derive_signal_ops!(MidiNote);
impl MidiNote {
  pub fn new(note: u8, ledger: &Arc<Mutex<MidiLedger>>) -> Self {
    Self {
      note,
      ledger: ledger.clone(),
    }
  }
}

impl Signal for MidiNote {
  fn sample(&mut self, _t: f64) -> f64 {
    let ledger = self.ledger.lock().unwrap();
    if ledger
      .notes
      .get(&self.note)
      .map(|note| note.down)
      .unwrap_or(false)
    {
      1.
    } else {
      0.
    }
  }
}

#[derive(Debug, Clone)]
pub struct MidiModWheel {
  ledger: Arc<Mutex<MidiLedger>>,
}
derive_signal_ops!(MidiModWheel);
impl MidiModWheel {
  pub fn new(ledger: &Arc<Mutex<MidiLedger>>) -> Self {
    Self {
      ledger: ledger.clone(),
    }
  }
}
impl Signal for MidiModWheel {
  fn sample(&mut self, _t: f64) -> f64 {
    self.ledger.lock().unwrap().mod_wheel
  }
}

#[derive(Debug, Clone)]
pub struct MidiPitchBend {
  ledger: Arc<Mutex<MidiLedger>>,
}
derive_signal_ops!(MidiPitchBend);
impl MidiPitchBend {
  pub fn new(ledger: Arc<Mutex<MidiLedger>>) -> Self {
    Self { ledger }
  }
}
impl Signal for MidiPitchBend {
  fn sample(&mut self, _t: f64) -> f64 {
    self.ledger.lock().unwrap().pitch_bend
  }
}
