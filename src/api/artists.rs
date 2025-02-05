use leptos::prelude::*;

use cfg_if::cfg_if;

use crate::models::frontend;
use crate::models::backend::Artist;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::util::database::get_db_conn;
        use diesel::prelude::*;
        use std::collections::HashMap;
        use server_fn::error::NoCustomError;
        use crate::models::backend::Album;
    }
}

/// Add an artist to the database
/// 
/// # Arguments
/// 
/// * `artist_name` - The name of the artist to add
/// 
/// # Returns
///  * `Result<(), Box<dyn Error>>` - A empty result if successful, or an error
/// 
#[server(endpoint = "artists/add-artist")]
pub async fn add_artist(artist_name: String) -> Result<(), ServerFnError> {
    use crate::schema::artists::dsl::*;
    use leptos::server_fn::error::NoCustomError;

    let new_artist = Artist {
        id: None,
        name: artist_name,
    };

    let db = &mut get_db_conn();
    diesel::insert_into(artists)
        .values(&new_artist)
        .execute(db)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error adding artist: {}", e)))?;
    
    Ok(())
}

#[server(endpoint = "artists/get")]
pub async fn get_artist_by_id(artist_id: i32) -> Result<Option<Artist>, ServerFnError> {
    use crate::schema::artists::dsl::*;
    use leptos::server_fn::error::NoCustomError;

    let db = &mut get_db_conn();
    let artist = artists
        .filter(id.eq(artist_id))
        .first::<Artist>(db)
        .optional()
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting artist: {}", e)))?;

    Ok(artist)
}

#[server(endpoint = "artists/top_songs")]
pub async fn top_songs_by_artist(artist_id: i32, limit: Option<i64>) -> Result<Vec<(frontend::Song, i64)>, ServerFnError> {
    use crate::models::backend::Song;
    use crate::auth::get_user;
    use crate::schema::*;
    use leptos::server_fn::error::NoCustomError;

    let user_id = get_user().await
        .map_err(|e| ServerFnError::ServerError::<NoCustomError>(format!("Error getting user: {}", e)))?.id.unwrap();

    let db = &mut get_db_conn();
    let song_play_counts: Vec<(i32, i64)> = 
        if let Some(limit) = limit {
            song_history::table
                .group_by(song_history::song_id)
                .select((song_history::song_id, diesel::dsl::count(song_history::id)))
                .left_join(song_artists::table.on(song_artists::song_id.eq(song_history::song_id)))
                .filter(song_artists::artist_id.eq(artist_id))
                .order_by(diesel::dsl::count(song_history::id).desc())
                .left_join(songs::table.on(songs::id.eq(song_history::song_id)))
                .limit(limit)
                .load(db)?
        } else {
            song_history::table
                .group_by(song_history::song_id)
                .select((song_history::song_id, diesel::dsl::count(song_history::id)))
                .left_join(song_artists::table.on(song_artists::song_id.eq(song_history::song_id)))
                .filter(song_artists::artist_id.eq(artist_id))
                .order_by(diesel::dsl::count(song_history::id).desc())
                .left_join(songs::table.on(songs::id.eq(song_history::song_id)))
                .load(db)?
        };

    let song_play_counts: HashMap<i32, i64> = song_play_counts.into_iter().collect();
    let top_song_ids: Vec<i32> = song_play_counts.iter().map(|(song_id, _)| *song_id).collect();

    let top_songs: Vec<(Song, Option<Album>, Option<Artist>, Option<(i32, i32)>, Option<(i32, i32)>)>
    = songs::table
        .filter(songs::id.eq_any(top_song_ids))
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
		.load(db)?;

    let mut top_songs_map: HashMap<i32, (frontend::Song, i64)> = HashMap::with_capacity(top_songs.len());

	for (song, album, artist, like, dislike) in top_songs {
		let song_id = song.id
			.ok_or(ServerFnError::ServerError::<NoCustomError>("Song id not found in database".to_string()))?;

		if let Some((stored_songdata, _)) = top_songs_map.get_mut(&song_id) {
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

			let plays = song_play_counts.get(&song_id)
				.ok_or(ServerFnError::ServerError::<NoCustomError>("Song id not found in history counts".to_string()))?;

			top_songs_map.insert(song_id, (songdata, *plays));
		}
	}

    let mut top_songs: Vec<(frontend::Song, i64)> = top_songs_map.into_iter().map(|(_, v)| v).collect();
    top_songs.sort_by(|(_, plays1), (_, plays2)| plays2.cmp(plays1));
    Ok(top_songs)
}

#[server(endpoint = "artists/albums")]
pub async fn albums_by_artist(artist_id: i32, limit: Option<i64>) -> Result<Vec<frontend::Album>, ServerFnError> {
    use crate::schema::*;

    let db = &mut get_db_conn();
    let album_ids: Vec<i32> = 
        if let Some(limit) = limit {
            albums::table
                .left_join(album_artists::table)
                .filter(album_artists::artist_id.eq(artist_id))
                .order_by(albums::release_date.desc())
                .limit(limit)
                .select(albums::id)
                .load(db)?
        } else {
            albums::table
                .left_join(album_artists::table)
                .filter(album_artists::artist_id.eq(artist_id))
                .order_by(albums::release_date.desc())
                .select(albums::id)
                .load(db)?
        };

    let mut albums_map: HashMap<i32, frontend::Album> = HashMap::with_capacity(album_ids.len());

    let album_artists: Vec<(Album, Artist)> = albums::table
        .filter(albums::id.eq_any(album_ids))
        .inner_join(album_artists::table.inner_join(artists::table).on(albums::id.eq(album_artists::album_id)))
        .select((albums::all_columns, artists::all_columns))
        .load(db)?;

    for (album, artist) in album_artists {
        let album_id = album.id
            .ok_or(ServerFnError::ServerError::<NoCustomError>("Album id not found in database".to_string()))?;

        if let Some(stored_album) = albums_map.get_mut(&album_id) {
            stored_album.artists.push(artist);
        } else {
            let albumdata = frontend::Album {
                id: album_id,
                title: album.title,
                artists: vec![artist],
                release_date: album.release_date,
                image_path: album.image_path.unwrap_or("/assets/images/placeholders/MusicPlaceholder.svg".to_string()),
            };

            albums_map.insert(album_id, albumdata);
        }
    }

    let mut albums: Vec<frontend::Album> = albums_map.into_iter().map(|(_, v)| v).collect();
    albums.sort_by(|a1, a2| a2.release_date.cmp(&a1.release_date));
    Ok(albums)
}
