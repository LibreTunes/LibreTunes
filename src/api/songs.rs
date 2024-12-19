use leptos::*;

use cfg_if::cfg_if;


cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::server_fn::error::NoCustomError;
		use crate::database::get_db_conn;
		use crate::auth::get_user;
	}
}

/// Like or unlike a song
#[server(endpoint = "songs/set_like")]
pub async fn set_like_song(song_id: i32, like: bool) -> Result<(), ServerFnError> {
	let user = get_user().await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error getting user: {}", e)))?;
	
	let db_con = &mut get_db_conn();

	user.set_like_song(song_id, like, db_con).await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error liking song: {}", e)))
}

/// Dislike or remove dislike from a song
#[server(endpoint = "songs/set_dislike")]
pub async fn set_dislike_song(song_id: i32, dislike: bool) -> Result<(), ServerFnError> {
	let user = get_user().await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error getting user: {}", e)))?;
	
	let db_con = &mut get_db_conn();

	user.set_dislike_song(song_id, dislike, db_con).await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error disliking song: {}", e)))
}

/// Get the like and dislike status of a song
#[server(endpoint = "songs/get_like_dislike")]
pub async fn get_like_dislike_song(song_id: i32) -> Result<(bool, bool), ServerFnError> {
	let user = get_user().await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error getting user: {}", e)))?;

	let db_con = &mut get_db_conn();

	// TODO this could probably be done more efficiently with a tokio::try_join, but 
	// doing so is much more complicated than it would initially seem

	let like = user.get_like_song(song_id, db_con).await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error getting song liked: {}", e)))?;
	let dislike = user.get_dislike_song(song_id, db_con).await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error getting song disliked: {}", e)))?;

	Ok((like, dislike))
}
