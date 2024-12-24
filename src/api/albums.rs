use leptos::prelude::*;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::database::get_db_conn;
        use diesel::prelude::*;
        use chrono::NaiveDate;
    }
}

/// Add an album to the database
/// 
/// # Arguments
/// 
/// * `album_title` - The name of the artist to add
/// * `release_data` - The release date of the album (Optional)
/// * `image_path` - The path to the album's image file (Optional)
/// 
/// # Returns
///  * `Result<(), Box<dyn Error>>` - A empty result if successful, or an error
/// 
#[server(endpoint = "albums/add-album")]
pub async fn add_album(album_title: String, release_date: Option<String>, image_path: Option<String>) -> Result<(), ServerFnError> {
    use crate::schema::albums::{self};
    use crate::models::Album;
    use leptos::server_fn::error::NoCustomError;
    
    let parsed_release_date = match release_date {
        Some(date) => {
            match NaiveDate::parse_from_str(&date.trim(), "%Y-%m-%d") {
                Ok(parsed_date) => Some(parsed_date),
                Err(_e) => return Err(ServerFnError::<NoCustomError>::ServerError("Invalid release date".to_string()))
            }
        },
        None => None
    };

    let image_path_arg = match image_path {
        Some(image_path) => {
            if image_path.is_empty() {
                None
            } else {
                Some(image_path)
            }
        },
        None => None
    };
    
    let new_album = Album {
        id: None,
        title: album_title,
        release_date: parsed_release_date,
        image_path: image_path_arg
    };

    let db = &mut get_db_conn();
    diesel::insert_into(albums::table)
        .values(&new_album)
        .execute(db)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error adding album: {}", e)))?;

    Ok(())
}
