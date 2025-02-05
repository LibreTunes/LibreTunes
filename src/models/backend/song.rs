use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
		use crate::util::database::*;
		use std::error::Error;
		use crate::models::backend::{Artist, Album};
	}
}

/// Model for a song
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::songs))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Clone, Serialize, Deserialize)]
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
	pub release_date: Option<NaiveDate>,
	/// The path to the song's audio file
	pub storage_path: String,
	/// The path to the song's image file
	pub image_path: Option<String>,
	/// The date the song was added to the database
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = NaiveDateTime))]
	pub added_date: Option<NaiveDateTime>,
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
