use leptos::*;
use crate::models::Playlist;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::database::get_db_conn;
        use diesel::prelude::*;
        use leptos_axum::extract;
        use axum_login::AuthSession;
        use crate::auth_backend::AuthBackend;
    }
}
/// Create a new, empty playlist in the database
/// 
/// # Arguments
/// 
/// * `new_playlist` - The new playlist
/// * `conn` - A mutable reference to a database connection
/// 	
/// # Returns
/// 
/// * `Result<(), Box<dyn Error>>` - A empty result if successful, or an error
/// 
#[server(endpoint = "playlists/create-playlist")]
pub async fn create_playlist(playlist_name: String)->Result<(), ServerFnError> {
    use crate::schema::playlists::dsl::*;
    use leptos::server_fn::error::NoCustomError;

    let auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

    //Ensure the playlist has no id
    let new_playlist = Playlist {
        id: None,
        name: playlist_name,
        user_id: auth_session.user.unwrap().id.expect("User has no id"),
    };

    let db_con = &mut get_db_conn();
    diesel::insert_into(playlists)
        .values(&new_playlist)
        .execute(db_con)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error creating playlist: {}", e)))?;

    Ok(())
}

/// Get all playlists for the current user
/// 
/// # Returns
/// 
/// * `Result<Vec<Playlist>, ServerFnError>` - A vector of playlists if successful, or an error
/// 
#[server(endpoint = "playlists/get-playlists")]
pub async fn get_playlists() -> Result<Vec<Playlist>, ServerFnError> {
    use crate::schema::playlists::dsl::*;
    use leptos::server_fn::error::NoCustomError;

    let auth_session = extract::<AuthSession<AuthBackend>>().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting auth session: {}", e)))?;

    let other_user_id = auth_session.user.unwrap().id.expect("User has no id");

    let db_con = &mut get_db_conn();
    let results = playlists
        .filter(user_id.eq(other_user_id))
        .load::<Playlist>(db_con)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting playlists: {}", e)))?;

    Ok(results)
}
/// Add a song to a playlist
/// 
/// # Arguments
/// 
/// * `playlist_id` - The id of the playlist
/// * `song_id` - The id of the song
/// 
/// # Returns
/// 
/// * `Result<(), ServerFnError>` - An empty result if successful, or an error
/// 
#[server(endpoint = "playlists/add-song")]
pub async fn add_song(new_playlist_id: Option<i32>, new_song_id: Option<i32>) -> Result<(), ServerFnError> {
    use crate::schema::playlist_songs::dsl::*;
    use leptos::server_fn::error::NoCustomError;

    let other_playlist_id = new_playlist_id.ok_or(ServerFnError::<NoCustomError>::ServerError("Playlist id must be present (Some) to add song".to_string()))?;

    let other_song_id = new_song_id.ok_or(ServerFnError::<NoCustomError>::ServerError("Song id must be present (Some) to add song".to_string()))?;

    let db_con = &mut get_db_conn();
    diesel::insert_into(playlist_songs)
        .values((playlist_id.eq(other_playlist_id), song_id.eq(other_song_id)))
        .execute(db_con)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error adding song to playlist: {}", e)))?;

    Ok(())
}
/// Get songs from a playlist
/// 
/// # Arguments
/// 
/// * `playlist_id` - The id of the playlist
/// 
/// # Returns
/// 
/// * `Result<Vec<Song>, ServerFnError>` - A vector of songs if successful, or an error
/// 
#[server(endpoint = "playlists/get-songs")]
pub async fn get_songs(new_playlist_id: Option<i32>) -> Result<Vec<crate::models::Song>, ServerFnError> {
    use crate::schema::playlist_songs::dsl::*;
    use crate::schema::songs::dsl::*;
    use leptos::server_fn::error::NoCustomError;

    let other_playlist_id = new_playlist_id.ok_or(ServerFnError::<NoCustomError>::ServerError("Playlist id must be present (Some) to get songs".to_string()))?;

    let db_con = &mut get_db_conn();
    let results = playlist_songs
        .inner_join(songs)
        .filter(playlist_id.eq(other_playlist_id))
        .select(songs::all_columns())
        .load(db_con)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting songs from playlist: {}", e)))?;

    Ok(results)
}
/// Remove a song from a playlist
/// 
/// # Arguments
/// 
/// * `playlist_id` - The id of the playlist
/// * `song_id` - The id of the song
/// 
/// # Returns
///    
/// * `Result<(), ServerFnError>` - An empty result if successful, or an error
///
#[server(endpoint = "playlists/remove-song")]
pub async fn remove_song(new_playlist_id: Option<i32>, new_song_id: Option<i32>) -> Result<(), ServerFnError> {
    use crate::schema::playlist_songs::dsl::*;
    use leptos::server_fn::error::NoCustomError;

    let other_playlist_id = new_playlist_id.ok_or(ServerFnError::<NoCustomError>::ServerError("Playlist id must be present (Some) to remove song".to_string()))?;

    let other_song_id = new_song_id.ok_or(ServerFnError::<NoCustomError>::ServerError("Song id must be present (Some) to remove song".to_string()))?;

    let db_con = &mut get_db_conn();
    diesel::delete(playlist_songs.filter(playlist_id.eq(other_playlist_id).and(song_id.eq(other_song_id))))
        .execute(db_con)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error removing song from playlist: {}", e)))?;

    Ok(())
} 