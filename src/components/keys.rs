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
    let handlePlayKey = move |note: StdScale| {
        if let Some(Ok((_stream_handle, sink))) = audio.read().as_ref() {
            play_sine_note(sink, note, Some(0.25));
        }
    };
    rsx! {
        div { id: "keys",
            div {
                class: "key",
                onclick: move |_event| { handlePlayKey(StdScale::C4) },
                p { "C" }
            }
            div {
                class: "key",
                onclick: move |_event| {
                    handlePlayKey(StdScale::D4);
                },
                p { "D" }
            }
            div {
                class: "key",
                onclick: move |_event| {
                    handlePlayKey(StdScale::E4);
                },
                p { "E" }
            }
            div { class: "key" }
        }
    }
}
