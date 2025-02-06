#![warn(
    unsafe_code,
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_line_after_outer_attr,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    keyword_idents,
    clippy::missing_const_for_fn,
    non_ascii_idents,
    noop_method_call,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::semicolon_if_nothing_returned,
    clippy::unseparated_literal_suffix,
    clippy::suspicious_operation_groupings,
    unused_import_braces,
    clippy::unused_self,
    clippy::use_debug,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
)]

#![allow(
    clippy::unused_unit,
    clippy::unit_arg,
    clippy::type_complexity,
)]

pub mod app;
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
