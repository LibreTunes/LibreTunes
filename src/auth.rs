use leptos::*;

use crate::models::User;

/// Create a new user and log them in
/// Takes in a NewUser struct, with the password in plaintext
/// Returns a Result with the error message if the user could not be created
#[server(endpoint = "signup")]
pub async fn signup(new_user: User) -> Result<(), ServerFnError> {
	use crate::users::create_user;

	use leptos::server_fn::error::NoCustomError;

	// Ensure the user has no id
	let new_user = User {
		id: None,
		..new_user
	};

	create_user(&new_user).await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error creating user: {}", e)))?;

	/*match extract::<HttpRequest>().await {
		Ok(request) => {
			match Identity::login(&request.extensions(), new_user.username.clone()) {
				Ok(_) => Ok(()),
				Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error logging in user: {}", e))),
			}
		},
		Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error getting request: {}", e))),
	}*/

	Ok(())
}

/// Log a user in
/// Takes in a username or email and a password in plaintext
/// Returns a Result with a boolean indicating if the login was successful
#[server(endpoint = "login")]
pub async fn login(username_or_email: String, password: String) -> Result<bool, ServerFnError> {
	use crate::users::validate_user;
	use leptos::server_fn::error::NoCustomError;

	let possible_user = validate_user(username_or_email, password).await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error validating user: {}", e)))?;

	let user = match possible_user {
		Some(user) => user,
		None => return Ok(false)
	};

	/*match extract::<HttpRequest>().await {
		Ok(request) => {
			match Identity::login(&request.extensions(), user.username.clone()) {
				Ok(_) => Ok(true),
				Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error logging in user: {}", e))),
			}
		}
		Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error getting request: {}", e))),
	}*/

	Ok(false)
}

/// Log a user out
/// Returns a Result with the error message if the user could not be logged out
#[server(endpoint = "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
	use leptos::server_fn::error::NoCustomError;

	/*match extract::<Option<Identity>>().await {
		Ok(Some(user)) => user.logout(),
		Ok(None) => {},
		Err(e) => return Err(ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e))),
	};*/

	Ok(())
}

