use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
		use crate::util::database::*;
		use std::error::Error;
		use crate::models::backend::{Song, HistoryEntry};
	}
}

// Model for a "User", used for querying the database
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
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = NaiveDateTime))]
	pub created_at: Option<NaiveDateTime>,
	/// Whether the user is an admin
	pub admin: bool,
}


impl User {
	/// Get the history of songs listened to by this user from the database
	/// 
	/// The returned history will be ordered by date in descending order,
	/// and a limit of N will select the N most recent entries.
	/// The `id` field of this user must be present (Some) to get history
	/// 
	/// # Arguments
	/// 
	/// * `limit` - An optional limit on the number of history entries to return
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Vec<HistoryEntry>, Box<dyn Error>>` -
	/// 	A result indicating success with a vector of history entries, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_history(self: &Self, limit: Option<i64>, conn: &mut PgPooledConn) ->
		Result<Vec<HistoryEntry>, Box<dyn Error>> {
		use crate::schema::song_history::dsl::*;

		let my_id = self.id.ok_or("Artist id must be present (Some) to get history")?;

		let my_history = 
		if let Some(limit) = limit {
			song_history
				.filter(user_id.eq(my_id))
				.order(date.desc())
				.limit(limit)
				.load(conn)?
		} else {
			song_history
				.filter(user_id.eq(my_id))
				.load(conn)?
		};

		Ok(my_history)
	}

	/// Get the history of songs listened to by this user from the database
	/// 
	/// The returned history will be ordered by date in descending order,
	/// and a limit of N will select the N most recent entries.
	/// The `id` field of this user must be present (Some) to get history
	/// 
	/// # Arguments
	/// 
	/// * `limit` - An optional limit on the number of history entries to return
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Vec<(SystemTime, Song)>, Box<dyn Error>>` -
	/// 	A result indicating success with a vector of listen dates and songs, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_history_songs(self: &Self, limit: Option<i64>, conn: &mut PgPooledConn) ->
		Result<Vec<(NaiveDateTime, Song)>, Box<dyn Error>> {
		use crate::schema::songs::dsl::*;
		use crate::schema::song_history::dsl::*;

		let my_id = self.id.ok_or("Artist id must be present (Some) to get history")?;

		let my_history =
		if let Some(limit) = limit {
			song_history
				.inner_join(songs)
				.filter(user_id.eq(my_id))
				.order(date.desc())
				.limit(limit)
				.select((date, songs::all_columns()))
				.load(conn)?
		} else {
			song_history
				.inner_join(songs)
				.filter(user_id.eq(my_id))
				.order(date.desc())
				.select((date, songs::all_columns()))
				.load(conn)?
		};

		Ok(my_history)
	}

	/// Add a song to this user's history in the database
	/// 
	/// The date of the history entry will be the current time
	/// The `id` field of this user must be present (Some) to add history
	/// 
	/// # Arguments
	/// 
	/// * `song_id` - The id of the song to add to this user's history
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<(), Box<dyn Error>>` - A result indicating success with an empty value, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn add_history(self: &Self, song_id: i32, conn: &mut PgPooledConn) -> Result<(), Box<dyn Error>> {
		use crate::schema::song_history;

		let my_id = self.id.ok_or("Artist id must be present (Some) to add history")?;

		diesel::insert_into(song_history::table)
			.values((song_history::user_id.eq(my_id), song_history::song_id.eq(song_id)))
			.execute(conn)?;

		Ok(())
	}

	/// Like or unlike a song for this user
	/// If likeing a song, remove dislike if it exists
	#[cfg(feature = "ssr")]
	pub async fn set_like_song(self: &Self, song_id: i32, like: bool, conn: &mut PgPooledConn) -> 
		Result<(), Box<dyn Error>> {
		use log::*;
		debug!("Setting like for song {} to {}", song_id, like);

		use crate::schema::song_likes;
		use crate::schema::song_dislikes;

		let my_id = self.id.ok_or("User id must be present (Some) to like/un-like a song")?;

		if like {
			diesel::insert_into(song_likes::table)
				.values((song_likes::song_id.eq(song_id), song_likes::user_id.eq(my_id)))
				.execute(conn)?;

			// Remove dislike if it exists
			diesel::delete(song_dislikes::table.filter(song_dislikes::song_id.eq(song_id)
				.and(song_dislikes::user_id.eq(my_id))))
				.execute(conn)?;
		} else {
			diesel::delete(song_likes::table.filter(song_likes::song_id.eq(song_id).and(song_likes::user_id.eq(my_id))))
				.execute(conn)?;
		}

		Ok(())
	}

	/// Get the like status of a song for this user
	#[cfg(feature = "ssr")]
	pub async fn get_like_song(self: &Self, song_id: i32, conn: &mut PgPooledConn) -> Result<bool, Box<dyn Error>> {
		use crate::schema::song_likes;

		let my_id = self.id.ok_or("User id must be present (Some) to get like status of a song")?;

		let like = song_likes::table
			.filter(song_likes::song_id.eq(song_id).and(song_likes::user_id.eq(my_id)))
			.first::<(i32, i32)>(conn)
			.optional()?
			.is_some();

		Ok(like)
	}

	/// Dislike or remove dislike from a song for this user
	/// If disliking a song, remove like if it exists
	#[cfg(feature = "ssr")]
	pub async fn set_dislike_song(self: &Self, song_id: i32, dislike: bool, conn: &mut PgPooledConn) -> 
		Result<(), Box<dyn Error>> {
		use log::*;
		debug!("Setting dislike for song {} to {}", song_id, dislike);

		use crate::schema::song_likes;
		use crate::schema::song_dislikes;

		let my_id = self.id.ok_or("User id must be present (Some) to dislike/un-dislike a song")?;

		if dislike {
			diesel::insert_into(song_dislikes::table)
				.values((song_dislikes::song_id.eq(song_id), song_dislikes::user_id.eq(my_id)))
				.execute(conn)?;

			// Remove like if it exists
			diesel::delete(song_likes::table.filter(song_likes::song_id.eq(song_id)
				.and(song_likes::user_id.eq(my_id))))
				.execute(conn)?;
		} else {
			diesel::delete(song_dislikes::table.filter(song_dislikes::song_id.eq(song_id)
				.and(song_dislikes::user_id.eq(my_id))))
				.execute(conn)?;
		}

		Ok(())
	}

	/// Get the dislike status of a song for this user
	#[cfg(feature = "ssr")]
	pub async fn get_dislike_song(self: &Self, song_id: i32, conn: &mut PgPooledConn) -> Result<bool, Box<dyn Error>> {
		use crate::schema::song_dislikes;

		let my_id = self.id.ok_or("User id must be present (Some) to get dislike status of a song")?;

		let dislike = song_dislikes::table
			.filter(song_dislikes::song_id.eq(song_id).and(song_dislikes::user_id.eq(my_id)))
			.first::<(i32, i32)>(conn)
			.optional()?
			.is_some();

		Ok(dislike)
	}
}
