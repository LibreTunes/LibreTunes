use leptos::prelude::*;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::server_fn::error::NoCustomError;
		use leptos_axum::extract;
		use axum_login::AuthSession;
		use crate::auth_backend::AuthBackend;
	}
}

use crate::models::backend::User;
use crate::users::UserCredentials;

/// Create a new user and log them in
/// Takes in a NewUser struct, with the password in plaintext
/// Returns a Result with the error message if the user could not be created
#[server(endpoint = "signup")]
pub async fn signup(new_user: User) -> Result<(), ServerFnError> {
	// Check LIBRETUNES_DISABLE_SIGNUP env var
	if std::env::var("LIBRETUNES_DISABLE_SIGNUP").is_ok_and(|v| v == "true") {
		return Err(ServerFnError::<NoCustomError>::ServerError("Signup is disabled".to_string()));
	}

	use crate::users::create_user;

	// Ensure the user has no id, and is not a self-proclaimed admin
	let new_user = User {
		id: None,
		admin: false,
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
pub async fn login(credentials: UserCredentials) -> Result<Option<User>, ServerFnError> {
	use crate::users::validate_user;

	let mut auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	let user = validate_user(credentials).await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error validating user: {}", e)))?;

	if let Some(mut user) = user {
		auth_session.login(&user).await
			.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error logging in user: {}", e)))?;

		user.password = None;
		Ok(Some(user))
	} else {
		Ok(None)
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

	leptos_axum::redirect("/login");
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
/// use leptos::prelude::*;
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
/// use leptos::prelude::*;
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

#[server(endpoint = "get_logged_in_user")]
pub async fn get_logged_in_user() -> Result<Option<User>, ServerFnError> {
	let auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	let user = auth_session.user.map(|mut user| {
		user.password = None;
		user
	});

	Ok(user)
}

/// Check if a user is an admin
/// Returns a Result with a boolean indicating if the user is logged in and an admin
#[server(endpoint = "check_admin")]
pub async fn check_admin() -> Result<bool, ServerFnError> {
	let auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

	Ok(auth_session.user.as_ref().map(|u| u.admin).unwrap_or(false))
}

/// Require that a user is logged in and an admin
/// Returns a Result with the error message if the user is not logged in or is not an admin
/// Intended to be used at the start of a protected route, to ensure the user is logged in and an admin:
/// ```rust
/// use leptos::prelude::*;
/// use libretunes::auth::require_admin;
/// #[server(endpoint = "protected_admin_route")]
/// pub async fn protected_admin_route() -> Result<(), ServerFnError> {
/// 	require_admin().await?;
/// 	// Continue with protected route
/// 	Ok(())
/// }
/// ```
#[cfg(feature = "ssr")]
pub async fn require_admin() -> Result<(), ServerFnError> {
	check_admin().await.and_then(|is_admin| {
		if is_admin {
			Ok(())
		} else {
			Err(ServerFnError::<NoCustomError>::ServerError(format!("Unauthorized")))
		}
	})
}
