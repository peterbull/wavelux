use rodio::{mixer, source::{Amplify, SawtoothWave, SineWave, Source, TakeDuration, TriangleWave}, OutputStream, Sink};
use core::fmt;
use std::{collections::HashMap, fmt::Formatter, thread::{self, sleep}, time::Duration};
use rand::{self, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
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
  Saw,
  Triangle,
}
impl fmt::Display for Waveform {
  fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
    match self {
      Waveform::Sine => write!(f, "Sine"),
      Waveform::Saw => write!(f,"Saw"),
      Waveform::Triangle => write!(f,"Triangle"),
    }
  }

}


pub struct AudioManager {
    notes: HashMap<StdScale, (Waveform, Sink)>,
    stream: OutputStream
}

impl AudioManager {
    pub fn new() -> Self {
        let stream = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let notes = HashMap::new();
        Self { stream, notes }
    }

 
    pub fn create_source(&self, frequency: f32, wave_type: &Waveform) -> Box<dyn Source<Item = f32> + Send> {
        match wave_type {
            Waveform::Sine => Box::new(
                SineWave::new(frequency)
                    .fade_in(Duration::from_millis(10))
                    .amplify(0.15)
            ),
            Waveform::Saw => Box::new(
                SawtoothWave::new(frequency)
                    .fade_in(Duration::from_millis(10))
                    .amplify(0.15)
            ),
            Waveform::Triangle => Box::new(
                TriangleWave::new(frequency)
                    .fade_in(Duration::from_millis(10))
                    .amplify(0.15)
            ),
        }
    }

    pub fn start_note(&mut self, note: StdScale,  wave_type: Option<Waveform>, sustain: Option<f32> ){
        let wave_type = wave_type.unwrap_or(Waveform::Sine);
        let frequency = note.frequency(); 
        let mut source = self.create_source(frequency, &wave_type);
        let sink = Sink::connect_new(&self.stream.mixer());
        if let Some(duration) = sustain {
          source = Box::new(source.take_duration(Duration::from_secs_f32(duration)))
        }
        println!("adding source ___________________________________");
        sink.append(source);
        if let Some((_, old_sink)) = self.notes.insert(note, (wave_type, sink)) {
            old_sink.stop();
        }
    }
    pub fn stop_note(&mut self, note: StdScale) {
      if let Some((wave_type, sink)) = self.notes.get(&note) {
        sink.stop();
      }
  }
}





