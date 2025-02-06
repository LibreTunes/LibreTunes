use leptos::prelude::*;
use leptos::logging::*;

use crate::models::frontend::PlayStatus;
use crate::models::backend::User;
use crate::api::auth::get_logged_in_user;

/// Global front-end state
/// Contains anything frequently needed across multiple components
/// Behaves like a singleton, in that `provide_context`/`expect_context` will
/// always return the same instance
#[derive(Clone)]
pub struct GlobalState {
	/// A resource that fetches the logged in user
	/// This will not automatically refetch, so any login/logout related code
	/// should call `refetch` on this resource
    pub logged_in_user: Resource<Option<User>>,

    /// The current play status
    pub play_status: RwSignal<PlayStatus>,
}

impl GlobalState {
    pub fn new() -> Self {
        let play_status = RwSignal::new(PlayStatus::default());

        let logged_in_user = Resource::new(|| (), |_| async {
            get_logged_in_user().await
                .inspect_err(|e| {
                    error!("Error getting logged in user: {:?}", e);
                })
                .ok()
                .flatten()
        });

        Self {
            logged_in_user,
            play_status,
        }
    }

    pub fn logged_in_user() -> Resource<Option<User>> {
        expect_context::<Self>().logged_in_user
    }

    pub fn play_status() -> RwSignal<PlayStatus> {
        expect_context::<Self>().play_status
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}

