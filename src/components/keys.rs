use dioxus::prelude::*;
use crate::{
    audio::{AudioManager, StdScale, Waveform},
    components::ToggleButton,
};


fn key_to_note(key: &str) -> Option<StdScale> {
    match key {
        "a" => Some(StdScale::C4),
        "s" => Some(StdScale::D4),
        "d" => Some(StdScale::E4),
        "f" => Some(StdScale::F4),
        "g" => Some(StdScale::G4),
        "h" => Some(StdScale::A4),
        "j" => Some(StdScale::B4),
        "k" => Some(StdScale::C5),
        "l" => Some(StdScale::D5),
        "w" => Some(StdScale::CSharp4),
        "e" => Some(StdScale::DSharp4),
        "t" => Some(StdScale::FSharp4),
        "y" => Some(StdScale::GSharp4),
        "u" => Some(StdScale::BFlat4),
        _ => None,
    }
}


#[component]
pub fn Keys() -> Element {
    let mut wave_type = use_signal(|| Waveform::Sine);
    let mut audio = use_context::<Signal<AudioManager>>();  
    let mut handle_playnote = move |note: StdScale, sustain: Option<f32>| {
            audio.write().start_note(note, Some(*wave_type.read()),sustain);
    };
 
    let mut handle_stopnote = move |note: StdScale| {
          audio.write().stop_note(note);
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
                            Waveform::Saw => Waveform::Triangle,
                            Waveform::Triangle => Waveform::Sine,
                        };
                        wave_type.set(new_wave);
                    },
                    "{wave_type.read()}"
                }
            }
            div {
                id: "keys",
                tabindex: "0",
                onkeydown: move |event| {
                    if event.is_auto_repeating() {
                        return;
                    }
                    if let Key::Character(ch) = event.key() {
                        if let Some(note) = key_to_note(ch.as_str()) {
                            handle_playnote(note, None);
                        }
                    }
                },
                onkeyup: move |event: Event<KeyboardData>| {
                    if let Key::Character(ch) = event.key() {
                        if let Some(note) = key_to_note(ch.as_str()) {
                            handle_stopnote(note);
                        }
                    }
                },

                div {
                    class: "key",
                    onclick: move |_event| { handle_playnote(StdScale::C4, Some(0.5)) },
                    p { "C" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| { handle_playnote(StdScale::CSharp4, Some(0.5)) },
                    p { "C#" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_playnote(StdScale::D4, Some(0.5));
                    },
                    p { "D" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| {
                        handle_playnote(StdScale::DSharp4, Some(0.5));
                    },
                    p { "D#" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_playnote(StdScale::E4, Some(0.5));
                    },
                    p { "E" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_playnote(StdScale::F4, Some(0.5));
                    },
                    p { "F" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| {
                        handle_playnote(StdScale::FSharp4, Some(0.5));
                    },
                    p { "F#" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_playnote(StdScale::G4, Some(0.5));
                    },
                    p { "G" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| {
                        handle_playnote(StdScale::GSharp4, Some(0.5));
                    },
                    p { "G#" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_playnote(StdScale::A4, Some(0.5));
                    },
                    p { "A" }
                }
                div {
                    class: "key-sharp",
                    onclick: move |_event| {
                        handle_playnote(StdScale::BFlat4, Some(0.5));
                    },
                    p { "Bb" }
                }
                div {
                    class: "key",
                    onclick: move |_event| {
                        handle_playnote(StdScale::B4, Some(0.5));
                    },
                    p { "B" }
                }
            }
        }
    }
}
