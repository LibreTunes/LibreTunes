use leptos::*;
use crate::models::Artist;
use crate::models::Song;


use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
	use crate::database::get_db_conn;
	use diesel::prelude::*;
}
}

/// Gets a Vector of Artists associated with a Song
/// 
/// # Arguments
/// `song_id_arg` - The id of the Song to get the Artists for
/// 
/// # Returns
/// A Result containing a Vector of Artists if the operation was successful, or an error if the operation failed
#[server(endpoint = "songs/get-artists")]
pub async fn get_artists(song_id_arg: Option<i32>) -> Result<Vec<Artist>, ServerFnError> {
	use crate::schema::artists::dsl::*;
	use crate::schema::song_artists::dsl::*;

	let my_id = song_id_arg.ok_or(ServerFnError::ServerError("Song id must be present (Some) to get artists".to_string()))?;

	let my_artists = artists
		.inner_join(song_artists)
		.filter(song_id.eq(my_id))
		.select(artists::all_columns())
		.load(&mut get_db_conn())?;

	Ok(my_artists)
}

/// Gets the song associated with a song id
/// 
/// # Arguments
/// `song_id_arg` - The id of the Song to get the song for
/// 
/// # Returns
/// A Result containing a Song if the operation was successful, or an error if the operation failed
#[server(endpoint = "songs/get-song")]
pub async fn get_song(song_id_arg: Option<i32>) -> Result<Song, ServerFnError> {
	use crate::schema::songs::dsl::*;

	let my_id = song_id_arg.ok_or(ServerFnError::ServerError("Song id must be present (Some) to get Song".to_string()))?;

	let mut my_song_vec: Vec<Song> = songs
		.filter(id.eq(my_id))
		.limit(1)
		.load(&mut get_db_conn())?;

	let my_song = my_song_vec.pop().ok_or(ServerFnError::ServerError("Song not found".to_string()))?;

	Ok(my_song)
}