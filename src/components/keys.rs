use std::collections::{hash_set, HashSet};

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

const PIANO_KEYS: &[(StdScale, &str, &str)] = &[
    (StdScale::C4, "key", "C"),
    (StdScale::CSharp4, "key-sharp", "C#"),
    (StdScale::D4, "key", "D"),
    (StdScale::DSharp4, "key-sharp", "D#"),
    (StdScale::E4, "key", "E"),
    (StdScale::F4, "key", "F"),
    (StdScale::FSharp4, "key-sharp", "F#"),
    (StdScale::G4, "key", "G"),
    (StdScale::GSharp4, "key-sharp", "G#"),
    (StdScale::A4, "key", "A"),
    (StdScale::BFlat4, "key-sharp", "Bb"),
    (StdScale::B4, "key", "B"),
];

#[component]
pub fn Keys() -> Element {
    let mut wave_type = use_signal(|| Waveform::Sine);
    let mut audio = use_context::<Signal<AudioManager>>();  
    let mut active_notes = use_signal(|| HashSet::<StdScale>::new());  
    let mut handle_playnote = move |note: StdScale, sustain: Option<f32>| {
            audio.write().start_note(note, Some(*wave_type.read()),sustain);
        active_notes.write().insert(note);  
    };

    let mut handle_stopnote = move |note: StdScale| {
          audio.write().stop_note(note);

        active_notes.write().remove(&note);  
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
                {
                    PIANO_KEYS
                        .iter()
                        .map(|(note, class, label)| {
                            let is_active = active_notes.read().contains(note);
                            let active_class = if is_active { " active" } else { "" };
                            rsx! {
                                div {
                                    key: "{label}",
                                    class: "{class}{active_class}",
                                    onmousedown: move |_| handle_playnote(*note, Some(0.5)),
                                    onmouseup: move |_| handle_stopnote(*note),
                                    onmouseleave: move |_| handle_stopnote(*note),
                                    p { "{label}" }
                                }
                            }
                        })
                }
            }
        }
    }
}
