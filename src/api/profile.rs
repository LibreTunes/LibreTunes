use leptos::prelude::*;
use server_fn::codec::{MultipartData, MultipartFormData};

use cfg_if::cfg_if;

use crate::models::frontend;
use chrono::NaiveDateTime;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use crate::api::auth::get_user;
		use server_fn::error::NoCustomError;

		use crate::util::database::get_db_conn;
		use diesel::prelude::*;
		use diesel::dsl::count;
		use crate::models::backend::{Album, Artist, Song, HistoryEntry};
		use crate::schema::*;

		use std::collections::HashMap;
	}
}

/// Handle a user uploading a profile picture. Converts the image to webp and saves it to the server.
#[server(input = MultipartFormData, endpoint = "/profile/upload_picture")]
pub async fn upload_picture(data: MultipartData) -> Result<(), ServerFnError> {
	// Safe to unwrap - "On the server side, this always returns Some(_). On the client side, always returns None."
	let mut data = data.into_inner().unwrap();

	let field = data.next_field().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting field: {}", e)))?
		.ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("No field found".to_string()))?;

	if field.name() != Some("picture") {
		return Err(ServerFnError::ServerError("Field name is not 'picture'".to_string()));
	}

	// Get user id from session
	let user = get_user().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e)))?;

	let user_id = user.id.ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("User has no id".to_string()))?;

	// Read the image, and convert it to webp
	use image_convert::{to_webp, WEBPConfig, ImageResource};

	let bytes = field.bytes().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting field bytes: {}", e)))?;

	let reader = std::io::Cursor::new(bytes);
	let image_source = ImageResource::from_reader(reader)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error creating image resource: {}", e)))?;

	let profile_picture_path = format!("assets/images/profile/{}.webp", user_id);
	let mut image_target = ImageResource::from_path(&profile_picture_path);
	to_webp(&mut image_target, &image_source, &WEBPConfig::new())
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error converting image to webp: {}", e)))?;

	Ok(())
}

/// Get a user's recent songs listened to
/// Optionally takes a limit parameter to limit the number of songs returned.
/// If not provided, all songs ever listend to are returned.
/// Returns a list of tuples with the date the song was listened to
/// and the song data, sorted by date (most recent first).
#[server(endpoint = "/profile/recent_songs")]
pub async fn recent_songs(for_user_id: i32, limit: Option<i64>) -> Result<Vec<(NaiveDateTime, frontend::Song)>, ServerFnError> {
	let viewing_user_id = get_user().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e)))?.id.unwrap();

	let mut db_con = get_db_conn();

	// Create an alias for the table so it can be referenced twice in the query
	let history2 = diesel::alias!(song_history as history2);

	// Get the ids of the most recent songs listened to
	let history_ids = history2
		.filter(history2.fields(song_history::user_id).eq(for_user_id))
		.order(history2.fields(song_history::date).desc())
		.select(history2.fields(song_history::id));

	let history_ids = if let Some(limit) = limit {
		history_ids.limit(limit).into_boxed()
	} else {
		history_ids.into_boxed()
	};

	// Take the history ids and get the song data for them
	let history: Vec<(HistoryEntry, Song, Option<Album>, Option<Artist>, Option<(i32, i32)>, Option<(i32, i32)>)>
	= song_history::table
		.filter(song_history::id.eq_any(history_ids))
		.inner_join(songs::table)
		.left_join(albums::table.on(songs::album_id.eq(albums::id.nullable())))
		.left_join(song_artists::table.inner_join(artists::table).on(songs::id.eq(song_artists::song_id)))
		.left_join(song_likes::table.on(songs::id.eq(song_likes::song_id).and(song_likes::user_id.eq(viewing_user_id))))
		.left_join(song_dislikes::table.on(
			songs::id.eq(song_dislikes::song_id).and(song_dislikes::user_id.eq(viewing_user_id))))
		.select((
			song_history::all_columns,
			songs::all_columns,
			albums::all_columns.nullable(),
			artists::all_columns.nullable(),
			song_likes::all_columns.nullable(),
			song_dislikes::all_columns.nullable(),
		))
		.load(&mut db_con)?;

	// Process the history data into a map of song ids to song data
	let mut history_songs: HashMap<i32, (NaiveDateTime, frontend::Song)> = HashMap::new();

	for (history, song, album, artist, like, dislike) in history {
		let song_id = history.song_id;

		if let Some((_, stored_songdata)) = history_songs.get_mut(&song_id) {
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
				album.as_ref().map(|album| album.image_path.clone()).flatten()
					.unwrap_or("/assets/images/placeholders/MusicPlaceholder.svg".to_string()));

			let songdata = frontend::Song {
				id: song_id,
				title: song.title,
				artists: artist.map(|artist| vec![artist]).unwrap_or_default(),
				album: album,
				track: song.track,
				duration: song.duration,
				release_date: song.release_date,
				song_path: song.storage_path,
				image_path: image_path,
				like_dislike: like_dislike,
				added_date: song.added_date.unwrap(),
			};

			history_songs.insert(song_id, (history.date, songdata));
		}
	}

	// Sort the songs by date
	let mut history_songs: Vec<(NaiveDateTime, frontend::Song)> = history_songs.into_values().collect();
	history_songs.sort_by(|a, b| b.0.cmp(&a.0));
	Ok(history_songs)
}

/// Get a user's top songs by play count from a date range
/// Optionally takes a limit parameter to limit the number of songs returned.
/// If not provided, all songs listened to in the date range are returned.
/// Returns a list of tuples with the play count and the song data, sorted by play count (most played first).
#[server(endpoint = "/profile/top_songs")]
pub async fn top_songs(for_user_id: i32, start_date: NaiveDateTime, end_date: NaiveDateTime, limit: Option<i64>)
	-> Result<Vec<(i64, frontend::Song)>, ServerFnError>
{	let viewing_user_id = get_user().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e)))?.id.unwrap();

	let mut db_con = get_db_conn();

	// Get the play count and ids of the songs listened to in the date range
	let history_counts: Vec<(i32, i64)> = 
		if let Some(limit) = limit {
			song_history::table
				.filter(song_history::date.between(start_date, end_date))
				.filter(song_history::user_id.eq(for_user_id))
				.group_by(song_history::song_id)
				.select((song_history::song_id, count(song_history::song_id)))
				.order(count(song_history::song_id).desc())
				.limit(limit)
				.load(&mut db_con)?
		} else {
			song_history::table
				.filter(song_history::date.between(start_date, end_date))
				.filter(song_history::user_id.eq(for_user_id))
				.group_by(song_history::song_id)
				.select((song_history::song_id, count(song_history::song_id)))
				.load(&mut db_con)?
		};

	let history_counts: HashMap<i32, i64> = history_counts.into_iter().collect();
	let history_song_ids = history_counts.iter().map(|(song_id, _)| *song_id).collect::<Vec<i32>>();

	// Get the song data for the songs listened to in the date range
	let history_songs: Vec<(Song, Option<Album>, Option<Artist>, Option<(i32, i32)>, Option<(i32, i32)>)>
	= songs::table
		.filter(songs::id.eq_any(history_song_ids))
		.left_join(albums::table.on(songs::album_id.eq(albums::id.nullable())))
		.left_join(song_artists::table.inner_join(artists::table).on(songs::id.eq(song_artists::song_id)))
		.left_join(song_likes::table.on(songs::id.eq(song_likes::song_id).and(song_likes::user_id.eq(viewing_user_id))))
		.left_join(song_dislikes::table.on(
			songs::id.eq(song_dislikes::song_id).and(song_dislikes::user_id.eq(viewing_user_id))))
		.select((
			songs::all_columns,
			albums::all_columns.nullable(),
			artists::all_columns.nullable(),
			song_likes::all_columns.nullable(),
			song_dislikes::all_columns.nullable(),
		))
		.load(&mut db_con)?;

	// Process the history data into a map of song ids to song data
	let mut history_songs_map: HashMap<i32, (i64, frontend::Song)> = HashMap::with_capacity(history_counts.len());

	for (song, album, artist, like, dislike) in history_songs {
		let song_id = song.id
			.ok_or(ServerFnError::ServerError::<NoCustomError>("Song id not found in database".to_string()))?;

		if let Some((_, stored_songdata)) = history_songs_map.get_mut(&song_id) {
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
				album.as_ref().map(|album| album.image_path.clone()).flatten()
					.unwrap_or("/assets/images/placeholders/MusicPlaceholder.svg".to_string()));

			let songdata = frontend::Song {
				id: song_id,
				title: song.title,
				artists: artist.map(|artist| vec![artist]).unwrap_or_default(),
				album: album,
				track: song.track,
				duration: song.duration,
				release_date: song.release_date,
				song_path: song.storage_path,
				image_path: image_path,
				like_dislike: like_dislike,
				added_date: song.added_date.unwrap(),
			};

			let plays = history_counts.get(&song_id)
				.ok_or(ServerFnError::ServerError::<NoCustomError>("Song id not found in history counts".to_string()))?;

			history_songs_map.insert(song_id, (*plays, songdata));
		}
	}

	// Sort the songs by play count
	let mut history_songs: Vec<(i64, frontend::Song)> = history_songs_map.into_values().collect();
	history_songs.sort_by(|a, b| b.0.cmp(&a.0));
	Ok(history_songs)
}

/// Get a user's top artists by play count from a date range
/// Optionally takes a limit parameter to limit the number of artists returned.
/// If not provided, all artists listened to in the date range are returned.
/// Returns a list of tuples with the play count and the artist data, sorted by play count (most played first).
#[server(endpoint = "/profile/top_artists")]
pub async fn top_artists(for_user_id: i32, start_date: NaiveDateTime, end_date: NaiveDateTime, limit: Option<i64>)
	-> Result<Vec<(i64, frontend::Artist)>, ServerFnError>
{
	let mut db_con = get_db_conn();

	let artist_counts: Vec<(i64, Artist)> = 
		if let Some(limit) = limit {
			song_history::table
				.filter(song_history::date.between(start_date, end_date))
				.filter(song_history::user_id.eq(for_user_id))
				.inner_join(song_artists::table.on(song_history::song_id.eq(song_artists::song_id)))
				.inner_join(artists::table.on(song_artists::artist_id.eq(artists::id)))
				.group_by(artists::id)
				.select((count(artists::id), artists::all_columns))
				.order(count(artists::id).desc())
				.limit(limit)
				.load(&mut db_con)?
		} else {
			song_history::table
				.filter(song_history::date.between(start_date, end_date))
				.filter(song_history::user_id.eq(for_user_id))
				.inner_join(song_artists::table.on(song_history::song_id.eq(song_artists::song_id)))
				.inner_join(artists::table.on(song_artists::artist_id.eq(artists::id)))
				.group_by(artists::id)
				.select((count(artists::id), artists::all_columns))
				.order(count(artists::id).desc())
				.load(&mut db_con)?
		};

	let artist_data: Vec<(i64, frontend::Artist)> = artist_counts.into_iter().map(|(plays, artist)| {
		(plays, frontend::Artist {
			id: artist.id.unwrap(),
			name: artist.name,
			image_path: format!("/assets/images/artists/{}.webp", artist.id.unwrap()),
		})
	}).collect();

	Ok(artist_data)
}
