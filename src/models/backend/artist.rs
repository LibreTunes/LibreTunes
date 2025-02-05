use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
		use crate::util::database::*;
		use std::error::Error;
		use crate::models::backend::{Album, Song};
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
