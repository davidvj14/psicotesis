pub mod app;
pub mod barrat;
pub mod barrat_components;
pub mod card_sorting;
pub mod card_sorting_components;
pub mod card_sorting_extras;
pub mod card_game;
pub mod card_game_components;
pub mod card_game_extras;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod questions;
pub mod questions_components;
pub mod barrat_extras;
pub mod extras;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
