use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		pub mod audio;
		pub mod require_auth;
	}
}

pub mod state;
