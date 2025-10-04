use rodio::source::{SineWave, Source};
use std::time::Duration;
use rand::{self, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug, EnumIter)]
pub enum StdScale {
    C4,
    CSharp4,
    D4,
    EFlat4,
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
    EFlat5,
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
            StdScale::EFlat4 => 311.13,
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
            StdScale::EFlat5 => 622.25,
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
pub fn play_sine_note(sink: &rodio::Sink, note: StdScale, sustain: Option<f32>) {
    let sustain = sustain.unwrap_or(0.25);
    let source = SineWave::new(note.frequency())
        .take_duration(Duration::from_secs_f32(sustain))
        .amplify(0.40);
    println!("clearing sink");
    sink.clear();
    println!("loading to sink");
    sink.append(source);
    if sink.is_paused() {
        sink.play();
    }
    println!("finished playing");
}
