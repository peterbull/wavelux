use core::fmt;
use std::collections::{hash_set, HashSet};

use crate::{
    audio::{AudioManager, StdScale, Waveform},
    components::ToggleButton,
};
use dioxus::prelude::*;

pub enum KeyType {
    White,
    Black,
}
impl KeyType {
    pub const fn class(&self) -> &str {
        match self {
            KeyType::White => "key",
            KeyType::Black => "key-sharp",
        }
    }
}
impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::White => write!(f, "{}", self.class()),
            KeyType::Black => write!(f, "{}", self.class()),
        }
    }
}

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

const PIANO_KEYS: &[(StdScale, KeyType)] = &[
    (StdScale::C4, KeyType::White),
    (StdScale::CSharp4, KeyType::Black),
    (StdScale::D4, KeyType::White),
    (StdScale::DSharp4, KeyType::Black),
    (StdScale::E4, KeyType::White),
    (StdScale::F4, KeyType::White),
    (StdScale::FSharp4, KeyType::Black),
    (StdScale::G4, KeyType::White),
    (StdScale::GSharp4, KeyType::Black),
    (StdScale::A4, KeyType::White),
    (StdScale::BFlat4, KeyType::Black),
    (StdScale::B4, KeyType::White),
];

#[component]
pub fn Keys() -> Element {
    let mut wave_type = use_signal(|| Waveform::Sine);
    let mut audio = use_context::<Signal<AudioManager>>();
    let mut active_notes = use_signal(|| HashSet::<StdScale>::new());
    let mut handle_playnote = move |note: StdScale, sustain: Option<f32>| {
        audio
            .write()
            .start_note(note, Some(*wave_type.read()), sustain);
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
                        let new_wave = match *wave_type.read() {
                            Waveform::Sine => Waveform::Saw,
                            Waveform::Saw => Waveform::Triangle,
                            Waveform::Triangle => Waveform::Square,
                            Waveform::Square => Waveform::Sine,
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
                        .map(|(note, class)| {
                            let is_active = active_notes.read().contains(note);
                            let active_class = if is_active { " active" } else { "" };
                            let note_name = note.to_string();
                            rsx! {
                                div {
                                    key: "{note_name}",
                                    class: "{class}{active_class}",
                                    onmousedown: move |_| handle_playnote(*note, Some(0.5)),
                                    onmouseup: move |_| handle_stopnote(*note),
                                    onmouseleave: move |_| handle_stopnote(*note),
                                    p { "{note_name}" }
                                }
                            }
                        })
                }
            }
        }
    }
}
