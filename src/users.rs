cfg_if::cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
		use crate::database::get_db_conn;

		use pbkdf2::{
			password_hash::{
				rand_core::OsRng,
				PasswordHasher, PasswordHash, SaltString, PasswordVerifier, Error
			},
			Pbkdf2
		};
	}
}

use leptos::*;
use serde::{Serialize, Deserialize};
use crate::models::User;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserCredentials {
	pub username_or_email: String,
	pub password: String
}

/// Get a user from the database by username or email
/// Returns a Result with the user if found, None if not found, or an error if there was a problem
#[cfg(feature = "ssr")]
pub async fn find_user(username_or_email: String) -> Result<Option<User>, ServerFnError> {
	use crate::schema::users::dsl::*;
	use leptos::server_fn::error::NoCustomError;

	// Look for either a username or email that matches the input, and return an option with None if no user is found
	let db_con = &mut get_db_conn();
	let user = users.filter(username.eq(username_or_email.clone())).or_filter(email.eq(username_or_email))
		.first::<User>(db_con).optional()
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user from database: {}", e)))?;

	Ok(user)
}

/// Create a new user in the database
/// Returns an empty Result if successful, or an error if there was a problem
#[cfg(feature = "ssr")]
pub async fn create_user(new_user: &User) -> Result<(), ServerFnError> {
	use crate::schema::users::dsl::*;
	use leptos::server_fn::error::NoCustomError;

	let new_password = new_user.password.clone()
		.ok_or(ServerFnError::<NoCustomError>::ServerError(format!("No password provided for user {}", new_user.username)))?;

	let salt = SaltString::generate(&mut OsRng);
	let password_hash = Pbkdf2.hash_password(new_password.as_bytes(), &salt)
		.map_err(|_| ServerFnError::<NoCustomError>::ServerError("Error hashing password".to_string()))?.to_string();
	
	let new_user = User {
		password: Some(password_hash),
		..new_user.clone()
	};
	
	let db_con = &mut get_db_conn();

	diesel::insert_into(users).values(&new_user).execute(db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error creating user: {}", e)))?;

	Ok(())
}

/// Validate a user's credentials
/// Returns a Result with the user if the credentials are valid, None if not valid, or an error if there was a problem
#[cfg(feature = "ssr")]
pub async fn validate_user(username_or_email: String, password: String) -> Result<Option<User>, ServerFnError> {
	use leptos::server_fn::error::NoCustomError;

	let db_user = find_user(username_or_email.clone()).await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user from database: {}", e)))?;

	// If the user is not found, return None
	let db_user = match db_user {
		Some(user) => user,
		None => return Ok(None)
	};

	let db_password = db_user.password.clone()
		.ok_or(ServerFnError::<NoCustomError>::ServerError(format!("No password found for user {}", db_user.username)))?;

	let password_hash = PasswordHash::new(&db_password)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error hashing supplied password: {}", e)))?;

	match Pbkdf2.verify_password(password.as_bytes(), &password_hash) {
		Ok(()) => {},
		Err(Error::Password) => {
			return Ok(None);
		},
		Err(e) => {
			return Err(ServerFnError::<NoCustomError>::ServerError(format!("Error verifying password: {}", e)));
		}
	}

	Ok(Some(db_user))
}

/// Get a user from the database by username or email
/// Returns a Result with the user if found, None if not found, or an error if there was a problem
#[server(endpoint = "get_user")]
pub async fn get_user(username_or_email: String) -> Result<Option<User>, ServerFnError> {
	let mut user = find_user(username_or_email).await?;

	// Remove the password hash before returning the user
	if let Some(user) = user.as_mut() {
		user.password = None;
	}

	Ok(user)
}
