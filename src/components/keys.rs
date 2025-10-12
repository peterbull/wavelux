use dioxus::prelude::*;
use crate::{
    audio::{start_note, StdScale, Waveform},
    components::ToggleButton,
};
#[component]
pub fn Keys() -> Element {
    let mut wave_type = use_signal(|| Waveform::Sine);
    let audio = use_resource(|| async {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
        Ok::<_, Box<dyn std::error::Error>>(stream_handle)
    });
    let handle_keydown = move |note: StdScale| {
        if let Some(Ok(stream_handle)) = audio.read().as_ref() {
            start_note(&stream_handle, note, Some(0.5), Some(*wave_type.read()));
        }
    };
    rsx! {
        div { class: "keyboard-container",
            div { id: "controls",
                ToggleButton {}
                button {
                    onclick: move |_event| {
                        let current = *wave_type.read();
                        let new_wave = match *wave_type.read() {
                            Waveform::Sine => Waveform::Saw,
                            Waveform::Saw => Waveform::Sine,
                        };
                        wave_type.set(new_wave);
                    },
                    "temp toggle"
                }
            }
            div {
                id: "keys",
                tabindex: "0",
                onkeydown: move |event| {
                    let key = event.key();
                    match key {
                        Key::Character(ch) => {
                            match ch.as_str() {
                                "a" => handle_keydown(StdScale::C4),
                                "s" => handle_keydown(StdScale::D4),
                                "d" => handle_keydown(StdScale::E4),
                                "f" => handle_keydown(StdScale::F4),
                                "g" => handle_keydown(StdScale::G4),
                                "h" => handle_keydown(StdScale::A4),
                                "j" => handle_keydown(StdScale::B4),
                                "k" => handle_keydown(StdScale::C5),
                                "l" => handle_keydown(StdScale::D5),
                                "w" => handle_keydown(StdScale::CSharp4),
                                "e" => handle_keydown(StdScale::DSharp4),
                                "t" => handle_keydown(StdScale::FSharp4),
                                "y" => handle_keydown(StdScale::GSharp4),
                                "u" => handle_keydown(StdScale::BFlat4),
                                _ => println!("other char"),
                            }
                        }
                        _ => println!("other key"),
                    }
                },
                div {
                    class: "key",
                    onclick: move |_event| { handle_keydown(StdScale::C4) },
                    p { "C" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| { handle_keydown(StdScale::CSharp4) },
                    p { "C#" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_keydown(StdScale::D4);
                    },
                    p { "D" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| {
                        handle_keydown(StdScale::DSharp4);
                    },
                    p { "D#" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_keydown(StdScale::E4);
                    },
                    p { "E" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_keydown(StdScale::F4);
                    },
                    p { "F" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| {
                        handle_keydown(StdScale::FSharp4);
                    },
                    p { "F#" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_keydown(StdScale::G4);
                    },
                    p { "G" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| {
                        handle_keydown(StdScale::GSharp4);
                    },
                    p { "G#" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_keydown(StdScale::A4);
                    },
                    p { "A" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| {
                        handle_keydown(StdScale::BFlat4);
                    },
                    p { "Bb" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_keydown(StdScale::B4);
                    },
                    p { "B" }
                }
            }
        }
    }
}
