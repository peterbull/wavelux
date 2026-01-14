use components::Keys;
use dioxus::prelude::*;
use strum::IntoEnumIterator;
mod audio;
mod components;
use crate::audio::AudioManager;
use dioxus_desktop::{Config, WindowBuilder};

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
fn main() {
    dioxus::launch(App);
}
/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    println!("Hello, world!");
    use_context_provider(|| Signal::new(AudioManager::new()));
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Keys {}
    }
}
