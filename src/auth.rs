use leptos::*;
use crate::models::User;

/// Create a new user and log them in
/// Takes in a NewUser struct, with the password in plaintext
/// Returns a Result with the error message if the user could not be created
#[server(endpoint = "signup")]
pub async fn signup(new_user: User) -> Result<(), ServerFnError> {
	use crate::users::create_user;

	use leptos_actix::extract;
	use actix_web::{HttpMessage, HttpRequest};
	use actix_identity::Identity;

	// Ensure the user has no id
	let new_user = User {
		id: None,
		..new_user
	};

	create_user(&new_user).await
		.map_err(|e| ServerFnError::ServerError(format!("Error creating user: {}", e)))?;

	extract(|request: HttpRequest| async move {
		Identity::login(&request.extensions(), new_user.username.clone())
	}).await??;

	Ok(())
}

/// Log a user in
/// Takes in a username or email and a password in plaintext
/// Returns a Result with a boolean indicating if the login was successful
#[server(endpoint = "login")]
pub async fn login(username_or_email: String, password: String) -> Result<bool, ServerFnError> {
	use crate::users::validate_user;
	use actix_web::{HttpMessage, HttpRequest};
	use actix_identity::Identity;
	use leptos_actix::extract;

	let possible_user = validate_user(username_or_email, password).await
		.map_err(|e| ServerFnError::ServerError(format!("Error validating user: {}", e)))?;

	let user = match possible_user {
		Some(user) => user,
		None => return Ok(false)
	};

	extract(|request: HttpRequest| async move {
		Identity::login(&request.extensions(), user.username.clone())
	}).await??;
	
	Ok(true)
}

/// Log a user out
/// Returns a Result with the error message if the user could not be logged out
#[server(endpoint = "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
	use leptos_actix::extract;
	use actix_identity::Identity;

	extract(|user: Option<Identity>| async move {
		if let Some(user) = user {
			user.logout();
		}
	}).await?;

	Ok(())
}

