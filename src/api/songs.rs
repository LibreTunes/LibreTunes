use leptos::prelude::*;

use cfg_if::cfg_if;

use crate::models::frontend;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use leptos::server_fn::error::NoCustomError;
		use crate::util::database::get_db_conn;
		use crate::api::auth::get_user;
		use crate::models::backend::{Song, Album, Artist};
		use diesel::prelude::*;
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

#[server(endpoint = "songs/get")]
pub async fn get_song_by_id(song_id: i32) -> Result<Option<frontend::Song>, ServerFnError> {
	use crate::schema::*;

	let user_id: i32 = get_user().await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error getting user: {}", e)))?.id.unwrap();

	let db_con = &mut get_db_conn();

	let song_parts: Vec<(Song, Option<Album>, Option<Artist>, Option<(i32, i32)>, Option<(i32, i32)>)>
	= songs::table
	.find(song_id)
	.left_join(albums::table.on(songs::album_id.eq(albums::id.nullable())))
	.left_join(song_artists::table.inner_join(artists::table).on(songs::id.eq(song_artists::song_id)))
	.left_join(song_likes::table.on(songs::id.eq(song_likes::song_id).and(song_likes::user_id.eq(user_id))))
	.left_join(song_dislikes::table.on(
		songs::id.eq(song_dislikes::song_id).and(song_dislikes::user_id.eq(user_id))))
	.select((
		songs::all_columns,
		albums::all_columns.nullable(),
		artists::all_columns.nullable(),
		song_likes::all_columns.nullable(),
		song_dislikes::all_columns.nullable(),
	))
	.load(db_con)?;

	let song = song_parts.first().cloned();
	let artists = song_parts.into_iter().filter_map(|(_, _, artist, _, _)| artist).collect::<Vec<_>>();

	match song {
		Some((song, album, _artist, like, dislike)) => {
			// Use song image path, or fall back to album image path, or fall back to placeholder
			let image_path = song.image_path.clone().unwrap_or_else(|| {
				album.as_ref().and_then(|album| album.image_path.clone()).unwrap_or(
					"/assets/images/placeholders/MusicPlaceholder.svg".to_string()
				)
			});

			Ok(Some(frontend::Song {
				id: song.id.unwrap(),
				title: song.title.clone(),
				artists,
				album: album.clone(),
				track: song.track,
				duration: song.duration,
				release_date: song.release_date,
				song_path: song.storage_path.clone(),
				image_path,
				like_dislike: Some((like.is_some(), dislike.is_some())),
				added_date: song.added_date.unwrap(),
			}))
		},
		None => Ok(None)
	}
}

#[server(endpoint = "songs/plays")]
pub async fn get_song_plays(song_id: i32) -> Result<i64, ServerFnError> {
	use crate::schema::*;

	let db_con = &mut get_db_conn();

	let plays = song_history::table
		.filter(song_history::song_id.eq(song_id))
		.count()
		.get_result::<i64>(db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::
			ServerError(format!("Error getting song plays: {}", e)))?;

	Ok(plays)
}

#[server(endpoint = "songs/my-plays")]
pub async fn get_my_song_plays(song_id: i32) -> Result<i64, ServerFnError> {
	use crate::schema::*;

	let user_id: i32 = get_user().await.map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error getting user: {}", e)))?.id.unwrap();

	let db_con = &mut get_db_conn();

	let plays = song_history::table
		.filter(song_history::song_id.eq(song_id).and(song_history::user_id.eq(user_id)))
		.count()
		.get_result::<i64>(db_con)
		.map_err(|e| ServerFnError::<NoCustomError>::
			ServerError(format!("Error getting song plays: {}", e)))?;

	Ok(plays)
}
