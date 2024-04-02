use leptos::*;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::server_fn::error::NoCustomError;
		use leptos_axum::extract;
		use axum_login::AuthSession;
		use crate::auth_backend::AuthBackend;
	}
}

use crate::models::User;
use crate::users::UserCredentials;

/// Create a new user and log them in
/// Takes in a NewUser struct, with the password in plaintext
/// Returns a Result with the error message if the user could not be created
#[server(endpoint = "signup")]
pub async fn signup(new_user: User) -> Result<(), ServerFnError> {
	use crate::users::create_user;

	// Ensure the user has no id
	let new_user = User {
		id: None,
		..new_user
	};

	create_user(&new_user).await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error creating user: {}", e)))?;

	let mut auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	let credentials = UserCredentials {
		username_or_email: new_user.username.clone(),
		password: new_user.password.clone().unwrap()
	};

	match auth_session.authenticate(credentials).await {
		Ok(Some(user)) => {
			auth_session.login(&user).await
				.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error logging in user: {}", e)))
		},
		Ok(None) => {
			Err(ServerFnError::<NoCustomError>::ServerError("Error authenticating user: User not found".to_string()))
		},
		Err(e) => {
			Err(ServerFnError::<NoCustomError>::ServerError(format!("Error authenticating user: {}", e)))
		}
	}
}

/// Log a user in
/// Takes in a username or email and a password in plaintext
/// Returns a Result with a boolean indicating if the login was successful
#[server(endpoint = "login")]
pub async fn login(credentials: UserCredentials) -> Result<bool, ServerFnError> {
	use crate::users::validate_user;

	let mut auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	let user = validate_user(credentials).await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error validating user: {}", e)))?;

	if let Some(user) = user {
		auth_session.login(&user).await
			.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error logging in user: {}", e)))?;
		Ok(true)
	} else {
		Ok(false)
	}
}

/// Log a user out
/// Returns a Result with the error message if the user could not be logged out
#[server(endpoint = "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
	let mut auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	auth_session.logout().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	Ok(())
}

