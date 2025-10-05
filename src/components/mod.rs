//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.
mod hero;
mod keys;
mod toggle_button;
pub use hero::Hero;
pub use keys::Keys;
pub use toggle_button::ToggleButton;
