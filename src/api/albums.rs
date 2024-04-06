use leptos::*;
use crate::models::Album;


use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
	use crate::database::get_db_conn;
	use diesel::prelude::*;
}
}

/// Gets the Album associated with an album id
/// 
/// # Arguments
/// `album_id_arg` The id of the Album to get the album for
/// 
/// # Returns
/// A Result containing an Album if the operation was successful, or an error if the operation failed
#[server(endpoint = "albums/get-album")]
pub async fn get_album(album_id_arg: Option<i32>) -> Result<Album, ServerFnError> {

	use crate::schema::albums::dsl::*;

	let my_id = album_id_arg.ok_or(ServerFnError::ServerError("Album id must be present (Some) to get Album".to_string()))?;

	let mut my_album_vec: Vec<Album> = albums
		.filter(id.eq(my_id))
		.limit(1)
		.load(&mut get_db_conn())?;

	let my_album = my_album_vec.pop().ok_or(ServerFnError::ServerError("Album not found".to_string()))?;

	Ok(my_album)
}