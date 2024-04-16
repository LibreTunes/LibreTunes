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
