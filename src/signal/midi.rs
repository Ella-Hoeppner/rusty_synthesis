use std::sync::Arc;
use std::sync::Mutex;

use crate::Signal;

use crate::MidiLedger;

#[derive(Debug, Clone)]
pub struct MidiNoteSignal {
  note: u8,
  ledger: Arc<Mutex<MidiLedger>>,
  on: bool,
}
impl MidiNoteSignal {
  pub fn new(note: u8, ledger: Arc<Mutex<MidiLedger>>) -> Self {
    Self {
      note,
      on: false,
      ledger,
    }
  }
  pub fn update_from_ledger(&mut self) {
    let ledger = self.ledger.lock().unwrap();
    self.on = ledger
      .notes
      .get(&self.note)
      .map(|note| note.down)
      .unwrap_or(false);
  }
}

impl Signal for MidiNoteSignal {
  fn sample(&mut self, _t: f64) -> f64 {
    self.update_from_ledger();
    if self.on {
      1.
    } else {
      0.
    }
  }
}
