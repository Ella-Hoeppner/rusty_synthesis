use anyhow::{anyhow, bail, Result};
use std::{
  collections::HashMap,
  fmt::Debug,
  io::{stdin, stdout, Write},
  sync::{Arc, Mutex},
};

use midir::{Ignore, MidiInput, MidiInputConnection};

#[derive(Debug, Clone)]
pub enum MidiEvent {
  NoteDown(u8),
  NoteUp(u8),
}

impl MidiEvent {
  pub fn from_message(bytes: &[u8]) -> Option<Self> {
    if bytes.len() == 3 {
      let status = bytes[0];
      let command = status >> 4;
      let channel = status & 0x0F;
      let note = bytes[1];
      match command {
        9 => Some(Self::NoteDown(note)),
        8 => Some(Self::NoteUp(note)),
        _ => None,
      }
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct NoteState {
  pub down: bool,
}

#[derive(Debug)]
pub struct MidiLedger {
  pub notes: HashMap<u8, NoteState>,
}

impl MidiLedger {
  pub fn new() -> Self {
    Self {
      notes: HashMap::new(),
    }
  }
  pub fn create() -> Arc<Mutex<Self>> {
    Arc::new(Mutex::new(Self::new()))
  }
}

pub struct MidiListener {
  ledger: Arc<Mutex<MidiLedger>>,
  input_connection: MidiInputConnection<Arc<Mutex<MidiLedger>>>,
}

impl MidiListener {
  pub fn start() -> Result<Self> {
    let ledger = MidiLedger::create();
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
      0 => bail!("no input port found"),
      1 => {
        println!(
          "Choosing the only available input port: {}",
          midi_in.port_name(&in_ports[0]).unwrap()
        );
        &in_ports[0]
      }
      _ => {
        println!("\nAvailable input ports:");
        for (i, p) in in_ports.iter().enumerate() {
          println!("{}: {}", i, midi_in.port_name(p).unwrap());
        }
        print!("Please select input port: ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        in_ports
          .get(input.trim().parse::<usize>()?)
          .ok_or(anyhow!("invalid input port selected"))?
      }
    };

    println!("Opening midi connection...");
    let in_port_name = midi_in.port_name(in_port)?;

    let input_connection = midi_in
      .connect(
        in_port,
        "midir-read-input",
        |_timestamp, message, ledger_mutex| {
          if let Some(event) = MidiEvent::from_message(message) {
            let mut ledger = ledger_mutex.lock().unwrap();
            match event {
              MidiEvent::NoteDown(note) => {
                ledger.notes.insert(note, NoteState { down: true });
              }
              MidiEvent::NoteUp(note) => {
                ledger.notes.insert(note, NoteState { down: false });
              }
            }
            println!("{:?}: {:?}", event, message);
          } else {
            println!("unrecognized midi message:: {:?}", message);
          }
        },
        ledger.clone(),
      )
      .map_err(|err| anyhow!(err.to_string()))?;

    println!(
      "Midi connection open, reading input from '{}'\n",
      in_port_name
    );

    Ok(Self {
      ledger,
      input_connection,
    })
  }
  pub fn get_ledger(&self) -> Arc<Mutex<MidiLedger>> {
    self.ledger.clone()
  }
}
