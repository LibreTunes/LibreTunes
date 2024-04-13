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
}

/// Model for an artist
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::artists))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize)]
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
}

/// Model for an album
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::albums))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize)]
pub struct Album {
	/// A unique id for the album
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The album's title
	pub title: String,
	/// The album's release date
	pub release_date: Option<Date>,
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
#[derive(Serialize, Deserialize, Clone)]
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
}
