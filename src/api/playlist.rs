use leptos::*;
use crate::playlistdata::PlaylistData;
use crate::songdata::SongData;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::server_fn::error::NoCustomError;
		use crate::database::get_db_conn;
	}
}

#[server(endpoint = "playlist/get")]
pub async fn get_playlist(id: i32) -> Result<PlaylistData, ServerFnError> {
    use crate::models::Playlist;
    let db_con = &mut get_db_conn();
    let playlist = Playlist::get_playlist_data(id,db_con)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting playlist: {}", e)))?;
    Ok(playlist)
}

