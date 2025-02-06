use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
		use crate::util::database::*;
		use std::error::Error;
		use crate::models::backend::{User, Artist, Song};
		use crate::models::frontend;
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
	pub release_date: Option<NaiveDate>,
	/// The path to the album's image file
	pub image_path: Option<String>,
}

impl Album {
	/// Obtain an album from its albumid
	/// # Arguments
	/// 
	/// * `album_id` - The id of the album to select
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Album, Box<dyn Error>>` - A result indicating success with the desired album, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_album_data(album_id: i32, conn: &mut PgPooledConn) -> Result<frontend::Album, Box<dyn Error>> {
		use crate::schema::*;

		let artist_list: Vec<Artist> = album_artists::table
			.filter(album_artists::album_id.eq(album_id))
			.inner_join(artists::table.on(album_artists::artist_id.eq(artists::id)))
			.select(
				artists::all_columns
			)
			.load(conn)?;

		// Get info of album
		let albuminfo = albums::table
			.filter(albums::id.eq(album_id))
			.first::<Album>(conn)?;

		let img = albuminfo.image_path.unwrap_or("/assets/images/placeholders/MusicPlaceholder.svg".to_string());

		let albumdata = frontend::Album {
			id: albuminfo.id.unwrap(),
			title: albuminfo.title,
			artists: artist_list,
			release_date: albuminfo.release_date,
			image_path: img
		};

		Ok(albumdata)
	}

	/// Obtain an album from its albumid
	/// # Arguments
	/// 
	/// * `album_id` - The id of the album to select
	/// * `conn` - A mutable reference to a database connection
	/// 
	/// # Returns
	/// 
	/// * `Result<Album, Box<dyn Error>>` - A result indicating success with the desired album, or an error
	/// 
	#[cfg(feature = "ssr")]
	pub fn get_song_data(album_id: i32, user_like_dislike: Option<User>, conn: &mut PgPooledConn) -> Result<Vec<frontend::Song>, Box<dyn Error>> {
		use crate::schema::*;
		use std::collections::HashMap;
		
		let song_list = if let Some(user_like_dislike) = user_like_dislike {
			let user_like_dislike_id = user_like_dislike.id.unwrap();
			let song_list: Vec<(Album, Option<Song>, Option<Artist>, Option<(i32, i32)>, Option<(i32, i32)>)> =
				albums::table
					.find(album_id)
					.left_join(songs::table.on(albums::id.nullable().eq(songs::album_id)))
					.left_join(song_artists::table.inner_join(artists::table).on(songs::id.eq(song_artists::song_id)))
					.left_join(song_likes::table.on(songs::id.eq(song_likes::song_id).and(song_likes::user_id.eq(user_like_dislike_id))))
					.left_join(song_dislikes::table.on(songs::id.eq(song_dislikes::song_id).and(song_dislikes::user_id.eq(user_like_dislike_id))))
					.select((
						albums::all_columns,
						songs::all_columns.nullable(),
						artists::all_columns.nullable(),
						song_likes::all_columns.nullable(),
						song_dislikes::all_columns.nullable()
					))
					.order(songs::track.asc())
					.load(conn)?;
			song_list
		} else {
			let song_list: Vec<(Album, Option<Song>, Option<Artist>)> =
				albums::table
					.find(album_id)
					.left_join(songs::table.on(albums::id.nullable().eq(songs::album_id)))
					.left_join(song_artists::table.inner_join(artists::table).on(songs::id.eq(song_artists::song_id)))
					.select((
						albums::all_columns,
						songs::all_columns.nullable(),
						artists::all_columns.nullable()
					))
					.order(songs::track.asc())
					.load(conn)?;

			let song_list:  Vec<(Album, Option<Song>, Option<Artist>, Option<(i32, i32)>, Option<(i32, i32)>)> =
				song_list.into_iter().map( |(album, song, artist)| (album, song, artist, None, None) ).collect();
			song_list
		};

		let mut album_songs: HashMap<i32, frontend::Song> = HashMap::with_capacity(song_list.len());
		
		for (album, song, artist, like, dislike) in song_list {
			if let Some(song) = song {				
				if let Some(stored_songdata) = album_songs.get_mut(&song.id.unwrap()) {
					// If the song is already in the map, update the artists
					if let Some(artist) = artist {
						stored_songdata.artists.push(artist);
					}
				} else {
					let like_dislike = match (like, dislike) {
						(Some(_), Some(_)) => Some((true, true)),
						(Some(_), None) => Some((true, false)),
						(None, Some(_)) => Some((false, true)),
						_ => None,
					};
		
					let image_path = song.image_path.unwrap_or(
						album.image_path.clone().unwrap_or("/assets/images/placeholders/MusicPlaceholder.svg".to_string()));
		
					let songdata = frontend::Song {
						id: song.id.unwrap(),
						title: song.title,
						artists: artist.map(|artist| vec![artist]).unwrap_or_default(),
						album: Some(album),
						track: song.track,
						duration: song.duration,
						release_date: song.release_date,
						song_path: song.storage_path,
						image_path: image_path,
						like_dislike: like_dislike,
						added_date: song.added_date.unwrap(),
					};
		
					album_songs.insert(song.id.unwrap(), songdata);
				}
			} 
		}
		
		// Sort the songs by date
		let mut songdata: Vec<frontend::Song> = album_songs.into_values().collect();
		songdata.sort_by(|a, b| a.track.cmp(&b.track));
		Ok(songdata)
	}
}
