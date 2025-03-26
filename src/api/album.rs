use leptos::prelude::*;
use crate::models::frontend;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
		use leptos::server_fn::error::NoCustomError;
		use crate::util::database::get_db_conn;
		use crate::models::backend;
	}
}

#[server(endpoint = "album/get")]
pub async fn get_album(id: i32) -> Result<Option<frontend::Album>, ServerFnError> {
	use crate::models::backend::Album;
	use crate::schema::*;

	let db_con = &mut get_db_conn();

	let album = albums::table
		.find(id)
		.first::<Album>(db_con)
		.optional()
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting album: {}", e)))?;

	let Some(album) = album else { return Ok(None) };

	let artists: Vec<backend::Artist> = album_artists::table
		.filter(album_artists::album_id.eq(id))
		.inner_join(artists::table.on(album_artists::artist_id.eq(artists::id)))
		.select(artists::all_columns)
		.load(db_con)?;

	let img = album.image_path.unwrap_or("/assets/images/placeholders/MusicPlaceholder.svg".to_string());

	let album = frontend::Album {
		id: album.id.unwrap(),
		title: album.title,
		artists,
		release_date: album.release_date,
		image_path: img
	};

	Ok(Some(album))
}

#[server(endpoint = "album/get_songs")]
pub async fn get_songs(id: i32) -> Result<Vec<frontend::Song>, ServerFnError> {
	use std::collections::HashMap;
	use crate::api::auth::get_logged_in_user;
	use crate::schema::*;

	let user = get_logged_in_user().await?;

	let db_con = &mut get_db_conn();
	
	let song_list = if let Some(user) = user {
		let user_id = user.id.unwrap();
		let song_list: Vec<(backend::Album, Option<backend::Song>, Option<backend::Artist>, Option<(i32, i32)>, Option<(i32, i32)>)> =
			albums::table
				.find(id)
				.left_join(songs::table.on(albums::id.nullable().eq(songs::album_id)))
				.left_join(song_artists::table.inner_join(artists::table).on(songs::id.eq(song_artists::song_id)))
				.left_join(song_likes::table.on(songs::id.eq(song_likes::song_id).and(song_likes::user_id.eq(user_id))))
				.left_join(song_dislikes::table.on(songs::id.eq(song_dislikes::song_id).and(song_dislikes::user_id.eq(user_id))))
				.select((
					albums::all_columns,
					songs::all_columns.nullable(),
					artists::all_columns.nullable(),
					song_likes::all_columns.nullable(),
					song_dislikes::all_columns.nullable()
				))
				.order(songs::track.asc())
				.load(db_con)?;
		song_list
	} else {
		let song_list: Vec<(backend::Album, Option<backend::Song>, Option<backend::Artist>)> =
			albums::table
				.find(id)
				.left_join(songs::table.on(albums::id.nullable().eq(songs::album_id)))
				.left_join(song_artists::table.inner_join(artists::table).on(songs::id.eq(song_artists::song_id)))
				.select((
					albums::all_columns,
					songs::all_columns.nullable(),
					artists::all_columns.nullable()
				))
				.order(songs::track.asc())
				.load(db_con)?;

		let song_list: Vec<(backend::Album, Option<backend::Song>, Option<backend::Artist>, Option<(i32, i32)>, Option<(i32, i32)>)> =
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
					image_path,
					like_dislike,
					added_date: song.added_date.unwrap(),
				};
	
				album_songs.insert(song.id.unwrap(), songdata);
			}
		} 
	}
	
	// Sort the songs by date
	let mut songs: Vec<frontend::Song> = album_songs.into_values().collect();
	songs.sort_by(|a, b| a.track.cmp(&b.track));

	Ok(songs)
}
