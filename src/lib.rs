#[cfg(feature = "ssr")]
pub mod backend;
pub mod frontend;
pub mod shared;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::frontend::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
