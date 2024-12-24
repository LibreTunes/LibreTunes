use chrono::NaiveDateTime;
use leptos::prelude::*;
use crate::models::HistoryEntry;
use crate::models::Song;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::server_fn::error::NoCustomError;
		use crate::database::get_db_conn;
		use crate::auth::get_user;
	}
}

/// Get the history of the current user.
#[server(endpoint = "history/get")]
pub async fn get_history(limit: Option<i64>) -> Result<Vec<HistoryEntry>, ServerFnError> {
	let user = get_user().await?;
	let db_con = &mut get_db_conn();
	let history = user.get_history(limit, db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting history: {}", e)))?;
	Ok(history)
}

/// Get the listen dates and songs of the current user.
#[server(endpoint = "history/get_songs")]
pub async fn get_history_songs(limit: Option<i64>) -> Result<Vec<(NaiveDateTime, Song)>, ServerFnError> {
	let user = get_user().await?;
	let db_con = &mut get_db_conn();
	let songs = user.get_history_songs(limit, db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting history songs: {}", e)))?;
	Ok(songs)
}

/// Add a song to the history of the current user.
#[server(endpoint = "history/add")]
pub async fn add_history(song_id: i32) -> Result<(), ServerFnError> {
	let user = get_user().await?;
	let db_con = &mut get_db_conn();
	user.add_history(song_id, db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error adding history: {}", e)))?;
	Ok(())
}
