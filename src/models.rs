use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use diesel::prelude::*;

// These "models" are used to represent the data in the database
// Diesel uses these models to generate the SQL queries that are used to interact with the database.
// These types are also used for API endpoints, for consistency. Because the file must be compiled
// for both the server and the client, we use the `cfg_attr` attribute to conditionally add
// diesel-specific attributes to the models when compiling for the server

/// Model for a "User", used for querying the database
/// Various fields are wrapped in Options, because they are not always wanted for inserts/retrieval
/// Using deserialize_as makes Diesel use the specified type when deserializing from the database,
/// and then call .into() to convert it into the Option
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::users))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
	/// A unique id for the user
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	// #[cfg_attr(feature = "ssr", diesel(skip_insertion))] // This feature is not yet released
	pub id: Option<i32>,
	/// The user's username
	pub username: String,
	/// The user's email
	pub email: String,
	/// The user's password, stored as a hash
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = String))]
	pub password: Option<String>,
	/// The time the user was created
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = SystemTime))]
	pub created_at: Option<SystemTime>,
}
