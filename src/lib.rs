pub mod app;
pub mod auth;
pub mod models;
pub mod pages;
pub mod components;
pub mod users;
pub mod error_template;
pub mod api;
pub mod util;

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    pub mod auth_backend;
    pub mod schema;
  }
}

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;

      console_error_panic_hook::set_once();

      leptos::mount::hydrate_body(App);
    }
}
}
