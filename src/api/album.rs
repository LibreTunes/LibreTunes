use leptos::*;
use crate::models::Album;
use crate::songdata::SongData;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::server_fn::error::NoCustomError;
		use crate::database::get_db_conn;
		use crate::auth::get_user;
	}
}

#[server(endpoint = "album/get")]
pub async fn get_album(id: i32) -> Result<Album, ServerFnError> {
	let db_con = &mut get_db_conn();
	let album = Album::get_album(id,db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting album: {}", e)))?;
	Ok(album)
}

#[server(endpoint = "album/get_songs")]
pub async fn get_songs(id: i32) -> Result<Vec<SongData>, ServerFnError> {
	use crate::auth::get_logged_in_user;
	let user = get_logged_in_user().await?;
	let db_con = &mut get_db_conn();
	// TODO: NEEDS SONG DATA QUERIES
	let songdata = Album::get_song_data(id,user,db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting song data: {}", e)))?;
	Ok(songdata)
}