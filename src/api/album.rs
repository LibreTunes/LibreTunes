use leptos::prelude::*;
use crate::models::frontend;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::server_fn::error::NoCustomError;
		use crate::util::database::get_db_conn;
	}
}

#[server(endpoint = "album/get")]
pub async fn get_album(id: i32) -> Result<frontend::Album, ServerFnError> {
	use crate::models::backend::Album;
	let db_con = &mut get_db_conn();
	let album = Album::get_album_data(id,db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting album: {}", e)))?;
	Ok(album)
}

#[server(endpoint = "album/get_songs")]
pub async fn get_songs(id: i32) -> Result<Vec<frontend::Song>, ServerFnError> {
	use crate::models::backend::Album;
	use crate::api::auth::get_logged_in_user;
	let user = get_logged_in_user().await?;
	let db_con = &mut get_db_conn();
	// TODO: NEEDS SONG DATA QUERIES
	let songdata = Album::get_song_data(id,user,db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting song data: {}", e)))?;
	Ok(songdata)
}