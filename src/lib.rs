pub mod app;
pub mod auth;
pub mod songdata;
pub mod playstatus;
pub mod playbar;
pub mod database;
pub mod queue;
pub mod song;
pub mod models;
pub mod pages;
pub mod components;
pub mod users;
pub mod search;
pub mod fileserv;
pub mod error_template;
pub mod api;
pub mod upload;
pub mod util;
pub mod api;

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

      leptos::mount_to_body(App);
    }
}
}
