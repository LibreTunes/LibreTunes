use leptos::*;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::database::get_db_conn;
        use diesel::prelude::*;
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
    use crate::models::Artist;
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
