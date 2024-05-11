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

/// Check if a user is logged in
/// Returns a Result with a boolean indicating if the user is logged in
#[server(endpoint = "check_auth")]
pub async fn check_auth() -> Result<bool, ServerFnError> {
	let auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	Ok(auth_session.user.is_some())
}

/// Require that a user is logged in
/// Returns a Result with the error message if the user is not logged in
/// Intended to be used at the start of a protected route, to ensure the user is logged in:
/// ```rust
/// use leptos::*;
/// use libretunes::auth::require_auth;
/// #[server(endpoint = "protected_route")]
/// pub async fn protected_route() -> Result<(), ServerFnError> {
/// 	require_auth().await?;
/// 	// Continue with protected route
/// 	Ok(())
/// }
/// ```
#[cfg(feature = "ssr")]
pub async fn require_auth() -> Result<(), ServerFnError> {
	check_auth().await.and_then(|logged_in| {
		if logged_in {
			Ok(())
		} else {
			Err(ServerFnError::<NoCustomError>::ServerError(format!("Unauthorized")))
		}
	})
}

/// Get the current logged-in user
/// Returns a Result with the user if they are logged in
/// Returns an error if the user is not logged in, or if there is an error getting the user
/// Intended to be used in a route to get the current user:
/// ```rust
/// use leptos::*;
/// use libretunes::auth::get_user;
/// #[server(endpoint = "user_route")]
/// pub async fn user_route() -> Result<(), ServerFnError> {
/// 	let user = get_user().await?;
/// 	println!("Logged in as: {}", user.username);
/// 	// Do something with the user
/// 	Ok(())
/// }
/// ```
#[cfg(feature = "ssr")]
pub async fn get_user() -> Result<User, ServerFnError> {
	let auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	auth_session.user.ok_or(ServerFnError::<NoCustomError>::ServerError("User not logged in".to_string()))
}
