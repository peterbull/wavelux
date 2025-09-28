// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use rodio::source::{SineWave, Source};
use std::time::Duration;

use components::Hero;
use rand::{self, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
/// Define a components module that contains all shared components for our app.
mod components;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Debug, EnumIter)]
enum StdScale {
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
    fn frequency(&self) -> f32 {
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

fn play_sine_note(sink: &rodio::Sink, note: StdScale, sustain: Option<f32>) {
    let sustain = sustain.unwrap_or(0.25);
    let source = SineWave::new(note.frequency())
        .take_duration(Duration::from_secs_f32(sustain))
        .amplify(0.40);
    sink.append(source);
    sink.sleep_until_end();
}

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // quick scale run to test
    for note in StdScale::iter() {
        let sustain = rand::random::<f32>();
        println!("note: {:#?} sustain: {:#?}", note, sustain);
        play_sine_note(&sink, note, Some(sustain));
    }

    println!("Hello, world!");
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        Hero {}

    }
}
