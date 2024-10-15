use std::time::SystemTime;
use time::Date;
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
		use crate::database::PgPooledConn;
		use std::error::Error;
	}
}

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
		Result<Vec<(SystemTime, Song)>, Box<dyn Error>> {
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

	/// Check if this user has listened to a song
	/// 
	/// The `id` field of this user must be present (Some) to check history
	/// 
	/// # Arguments
	/// 
	/// * `song_id` - The id of the song to check if this user has listened to
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<bool, Box<dyn Error>>` - A result indicating success with a boolean value, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn has_listened_to(self: &Self, song_id: i32, conn: &mut PgPooledConn) -> Result<bool, Box<dyn Error>> {
		use crate::schema::song_history::{self, user_id};

		let my_id = self.id.ok_or("Artist id must be present (Some) to check history")?;

		let has_listened = song_history::table
			.filter(user_id.eq(my_id))
			.filter(song_history::song_id.eq(song_id))
			.first::<HistoryEntry>(conn)
			.optional()?
			.is_some();

		Ok(has_listened)
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

	/// Get songs liked by this user
	#[cfg(feature = "ssr")]
	pub async fn get_liked_songs(self: &Self, conn: &mut PgPooledConn) -> Result<Vec<Song>, Box<dyn Error>> {
		use crate::schema::songs::dsl::*;
		use crate::schema::song_likes::dsl::*;

		let my_id = self.id.ok_or("User id must be present (Some) to get liked songs")?;

		let my_songs = songs
			.inner_join(song_likes)
			.filter(user_id.eq(my_id))
			.select(songs::all_columns())
			.load(conn)?;

		Ok(my_songs)
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

	/// Get songs disliked by this user
	#[cfg(feature = "ssr")]
	pub async fn get_disliked_songs(self: &Self, conn: &mut PgPooledConn) -> Result<Vec<Song>, Box<dyn Error>> {
		use crate::schema::songs::dsl::*;
		use crate::schema::song_likes::dsl::*;

		let my_id = self.id.ok_or("User id must be present (Some) to get disliked songs")?;

		let my_songs = songs
			.inner_join(song_likes)
			.filter(user_id.eq(my_id))
			.select(songs::all_columns())
			.load(conn)?;

		Ok(my_songs)
	}
}

/// Model for an artist
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable, Identifiable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::artists))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artist {
	/// A unique id for the artist
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The artist's name
	pub name: String,
}

impl Artist {
	/// Add an album to this artist in the database
	/// 
	/// # Arguments
	/// 
	/// * `new_album_id` - The id of the album to add to this artist
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<(), Box<dyn Error>>` - A result indicating success with an empty value, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn add_album(self: &Self, new_album_id: i32, conn: &mut PgPooledConn) -> Result<(), Box<dyn Error>> {
		use crate::schema::album_artists::dsl::*;

		let my_id = self.id.ok_or("Artist id must be present (Some) to add an album")?;

		diesel::insert_into(album_artists)
			.values((album_id.eq(new_album_id), artist_id.eq(my_id)))
			.execute(conn)?;

		Ok(())
	}

	/// Get albums by artist from the database
	/// 
	/// The `id` field of this artist must be present (Some) to get albums
	/// 
	/// # Arguments
	/// 
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Vec<Album>, Box<dyn Error>>` - A result indicating success with a vector of albums, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_albums(self: &Self, conn: &mut PgPooledConn) -> Result<Vec<Album>, Box<dyn Error>> {
		use crate::schema::albums::dsl::*;
		use crate::schema::album_artists::dsl::*;

		let my_id = self.id.ok_or("Artist id must be present (Some) to get albums")?;

		let my_albums = albums
			.inner_join(album_artists)
			.filter(artist_id.eq(my_id))
			.select(albums::all_columns())
			.load(conn)?;

		Ok(my_albums)
	}

	/// Add a song to this artist in the database
	/// 
	/// The `id` field of this artist must be present (Some) to add a song
	/// 
	/// # Arguments
	/// 
	/// * `new_song_id` - The id of the song to add to this artist
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<(), Box<dyn Error>>` - A result indicating success with an empty value, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn add_song(self: &Self, new_song_id: i32, conn: &mut PgPooledConn) -> Result<(), Box<dyn Error>> {
		use crate::schema::song_artists::dsl::*;

		let my_id = self.id.ok_or("Artist id must be present (Some) to add an album")?;

		diesel::insert_into(song_artists)
			.values((song_id.eq(new_song_id), artist_id.eq(my_id)))
			.execute(conn)?;

		Ok(())
	}

	/// Get songs by this artist from the database
	/// 
	/// The `id` field of this artist must be present (Some) to get songs
	/// 
	/// # Arguments
	/// 
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Vec<Song>, Box<dyn Error>>` - A result indicating success with a vector of songs, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_songs(self: &Self, conn: &mut PgPooledConn) -> Result<Vec<Song>, Box<dyn Error>> {
		use crate::schema::songs::dsl::*;
		use crate::schema::song_artists::dsl::*;

		let my_id = self.id.ok_or("Artist id must be present (Some) to get songs")?;

		let my_songs = songs
			.inner_join(song_artists)
			.filter(artist_id.eq(my_id))
			.select(songs::all_columns())
			.load(conn)?;

		Ok(my_songs)
	}

	/// Display a list of artists as a string.
	/// 
	/// For one artist, displays [artist1]. For two artists, displays [artist1] & [artist2].
	/// For three or more artists, displays [artist1], [artist2], & [artist3].
	pub fn display_list(artists: &Vec<Artist>) -> String {
		let mut artist_list = String::new();

		for (i, artist) in artists.iter().enumerate() {
			if i == 0 {
				artist_list.push_str(&artist.name);
			} else if i == artists.len() - 1 {
				artist_list.push_str(&format!(" & {}", artist.name));
			} else {
				artist_list.push_str(&format!(", {}", artist.name));
			}
		}

		artist_list
	}
}

/// Model for an album
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable, Identifiable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::albums))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Album {
	/// A unique id for the album
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The album's title
	pub title: String,
	/// The album's release date
	pub release_date: Option<Date>,
	/// The path to the album's image file
	pub image_path: Option<String>,
}

impl Album {
	/// Add an artist to this album in the database
	/// 
	/// The `id` field of this album must be present (Some) to add an artist
	/// 
	/// # Arguments
	/// 
	/// * `new_artist_id` - The id of the artist to add to this album
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<(), Box<dyn Error>>` - A result indicating success with an empty value, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn add_artist(self: &Self, new_artist_id: i32, conn: &mut PgPooledConn) -> Result<(), Box<dyn Error>> {
		use crate::schema::album_artists::dsl::*;

		let my_id = self.id.ok_or("Album id must be present (Some) to add an artist")?;

		diesel::insert_into(album_artists)
			.values((album_id.eq(my_id), artist_id.eq(new_artist_id)))
			.execute(conn)?;

		Ok(())
	}

	/// Get songs by this artist from the database
	/// 
	/// The `id` field of this album must be present (Some) to get songs
	/// 
	/// # Arguments
	/// 
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Vec<Song>, Box<dyn Error>>` - A result indicating success with a vector of songs, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_songs(self: &Self, conn: &mut PgPooledConn) -> Result<Vec<Song>, Box<dyn Error>> {
		use crate::schema::songs::dsl::*;
		use crate::schema::song_artists::dsl::*;

		let my_id = self.id.ok_or("Album id must be present (Some) to get songs")?;

		let my_songs = songs
			.inner_join(song_artists)
			.filter(album_id.eq(my_id))
			.select(songs::all_columns())
			.load(conn)?;

		Ok(my_songs)
	}
}

/// Model for a song
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::songs))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize)]
pub struct Song {
	/// A unique id for the song
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The song's title
	pub title: String,
	/// The album the song is from
	pub album_id: Option<i32>,
	/// The track number of the song on the album
	pub track: Option<i32>,
	/// The duration of the song in seconds
	pub duration: i32,
	/// The song's release date
	pub release_date: Option<Date>,
	/// The path to the song's audio file
	pub storage_path: String,
	/// The path to the song's image file
	pub image_path: Option<String>,
}

impl Song {
	/// Add an artist to this song in the database
	/// 
	/// The `id` field of this song must be present (Some) to add an artist
	/// 
	/// # Arguments
	/// 
	/// * `new_artist_id` - The id of the artist to add to this song
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Vec<Artist>, Box<dyn Error>>` - A result indicating success with an empty value, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_artists(self: &Self, conn: &mut PgPooledConn) -> Result<Vec<Artist>, Box<dyn Error>> {
		use crate::schema::artists::dsl::*;
		use crate::schema::song_artists::dsl::*;

		let my_id = self.id.ok_or("Song id must be present (Some) to get artists")?;

		let my_artists = artists
			.inner_join(song_artists)
			.filter(song_id.eq(my_id))
			.select(artists::all_columns())
			.load(conn)?;

		Ok(my_artists)
	}

	/// Get the album for this song from the database
	/// 
	/// # Arguments
	/// 
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Option<Album>, Box<dyn Error>>` - A result indicating success with an album, or None if
	/// the song does not have an album, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_album(self: &Self, conn: &mut PgPooledConn) -> Result<Option<Album>, Box<dyn Error>> {
		use crate::schema::albums::dsl::*;

		if let Some(album_id) = self.album_id {
			let my_album = albums
				.filter(id.eq(album_id))
				.first::<Album>(conn)?;

			Ok(Some(my_album))
		} else {
			Ok(None)
		}
	}
}

/// Model for a history entry
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::song_history))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize)]
pub struct HistoryEntry {
	/// A unique id for the history entry
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The id of the user who listened to the song
	pub user_id: i32,
	/// The date the song was listened to
	pub date: SystemTime,
	/// The id of the song that was listened to
	pub song_id: i32,
}
