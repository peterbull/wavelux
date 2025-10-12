use rodio::{mixer, source::{Amplify, SawtoothWave, SineWave, Source, TakeDuration}, OutputStream, Sink};
use std::{collections::HashMap, thread::{self, sleep}, time::Duration};
use rand::{self, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
pub enum StdScale {
    C4,
    CSharp4,
    D4,
    DSharp4,
    E4,
    F4,
    FSharp4,
    G4,
    GSharp4,
    A4,
    BFlat4,
    B4,
    C5,
    CSharp5,
    D5,
    DSharp5,
    E5,
    F5,
    FSharp5,
    G5,
    GSharp5,
    A5,
    BFlat5,
    B5,
}
impl StdScale {
    pub fn frequency(&self) -> f32 {
        match self {
            StdScale::C4 => 261.63,
            StdScale::CSharp4 => 277.18,
            StdScale::D4 => 293.66,
            StdScale::DSharp4 => 311.13,
            StdScale::E4 => 329.63,
            StdScale::F4 => 349.23,
            StdScale::FSharp4 => 369.99,
            StdScale::G4 => 392.00,
            StdScale::GSharp4 => 415.30,
            StdScale::A4 => 440.00,
            StdScale::BFlat4 => 466.16,
            StdScale::B4 => 493.88,
            StdScale::C5 => 523.25,
            StdScale::CSharp5 => 554.37,
            StdScale::D5 => 587.33,
            StdScale::DSharp5 => 622.25,
            StdScale::E5 => 659.25,
            StdScale::F5 => 698.46,
            StdScale::FSharp5 => 739.99,
            StdScale::G5 => 783.99,
            StdScale::GSharp5 => 830.61,
            StdScale::A5 => 880.00,
            StdScale::BFlat5 => 932.33,
            StdScale::B5 => 987.77,
        }
    }
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Waveform {
  Sine,
  Saw
}

pub struct AudioManager {
    notes: HashMap<StdScale, Sink>,
    stream: OutputStream
}

impl AudioManager {
    pub fn new() -> Self {
        let stream = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let notes = HashMap::new();
        Self { stream, notes }
    }

    pub fn start_note(&mut self, note: StdScale,  wave_type: Option<Waveform>) {
        let sink = Sink::connect_new(&self.stream.mixer());
        let wave_type = wave_type.unwrap_or(Waveform::Sine);
        let source: Box<dyn Source<Item = f32> + Send> = match wave_type {
            Waveform::Sine => Box::new(SineWave::new(note.frequency())
                .amplify(0.2)),
            Waveform::Saw => Box::new(SawtoothWave::new(note.frequency())
                .amplify(0.2)),
        };
        println!("adding source ___________________________________");
        sink.append(source);
        if let Some(old_sink) = self.notes.insert(note, sink) {
            old_sink.stop();
        }
    }
    pub fn stop_note(&mut self, note: StdScale) {
    if let Some(sink) = self.notes.get(&note) {
      sink.stop()
    }
  }
}





