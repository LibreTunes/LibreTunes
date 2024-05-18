use axum_login::{AuthnBackend, AuthUser, UserId};
use crate::users::UserCredentials;
use leptos::server_fn::error::ServerFnErrorErr;

use crate::models::User;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use async_trait::async_trait;
	}
}

impl AuthUser for User {
	type Id = i32;

	// TODO: Ideally, we shouldn't have to unwrap here

	fn id(&self) -> Self::Id {
		self.id.unwrap()
	}

	fn session_auth_hash(&self) -> &[u8] {
		self.password.as_ref().unwrap().as_bytes()
	}
}

#[derive(Clone)]
pub struct AuthBackend;

#[cfg(feature = "ssr")]
#[async_trait]
impl AuthnBackend for AuthBackend {
	type User = User;
	type Credentials = UserCredentials;
	type Error = ServerFnErrorErr;

	async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
		crate::users::validate_user(creds).await
			.map_err(|e| ServerFnErrorErr::ServerError(format!("Error validating user: {}", e)))
	}

	async fn get_user(&self, user_id: &UserId<Self>) ->  Result<Option<Self::User>, Self::Error> {
		crate::users::find_user_by_id(*user_id).await
			.map_err(|e| ServerFnErrorErr::ServerError(format!("Error getting user: {}", e)))
	}
}
