use dioxus::prelude::*;
use crate::audio::{play_sine_note, StdScale};
#[component]
pub fn Keys() -> Element {
    let audio = use_resource(|| async {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        Ok::<_, Box<dyn std::error::Error>>((stream_handle, sink))
    });
    let handle_play_key = move |note: StdScale| {
        if let Some(Ok((_stream_handle, sink))) = audio.read().as_ref() {
            println!("playing key in sink: {:#?}", sink.len());
            play_sine_note(sink, note, Some(0.1));
        }
    };
    rsx! {
        div {
            id: "keys",
            tabindex: "0",
            onkeydown: move |event| {
                let key = event.key();
                match key {
                    Key::Character(ch) => {
                        match ch.as_str() {
                            "a" => handle_play_key(StdScale::C4),
                            "s" => handle_play_key(StdScale::D4),
                            "d" => handle_play_key(StdScale::E4),
                            "f" => handle_play_key(StdScale::F4),
                            "g" => handle_play_key(StdScale::G4),
                            "h" => handle_play_key(StdScale::A4),
                            "j" => handle_play_key(StdScale::B4),
                            "k" => handle_play_key(StdScale::C5),
                            "l" => handle_play_key(StdScale::D5),
                            "w" => handle_play_key(StdScale::CSharp4),
                            "e" => handle_play_key(StdScale::EFlat4),
                            "t" => handle_play_key(StdScale::FSharp4),
                            "y" => handle_play_key(StdScale::GSharp4),
                            "u" => handle_play_key(StdScale::BFlat4),
                            _ => println!("other char"),
                        }
                    }
                    _ => println!("other key"),
                }
            },
            div {
                class: "key",
                onclick: move |_event| { handle_play_key(StdScale::C4) },
                p { "C" }
            }
            div {
                class: "key-sharp",
                onclick: move |_event| { handle_play_key(StdScale::CSharp4) },
                p { "C#" }
            }
            div {
                class: "key",
                onclick: move |_event| {
                    handle_play_key(StdScale::D4);
                },
                p { "D" }
            }
            div {
                class: "key",
                onclick: move |_event| {
                    handle_play_key(StdScale::E4);
                },
                p { "E" }
            }
            div {
                class: "key",
                onclick: move |_event| {
                    handle_play_key(StdScale::F4);
                },
                p { "F" }
            }
            div {
                class: "key",
                onclick: move |_event| {
                    handle_play_key(StdScale::G4);
                },
                p { "G" }
            }
            div {
                class: "key",
                onclick: move |_event| {
                    handle_play_key(StdScale::A4);
                },
                p { "A" }
            }
            div {
                class: "key",
                onclick: move |_event| {
                    handle_play_key(StdScale::B4);
                },
                p { "B" }
            }
        }
    }
}
