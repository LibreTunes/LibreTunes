use leptos::*;
use crate::models::{Artist, Album, Song};

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
	use diesel::sql_types::*;
	use diesel::*;
	use diesel::pg::Pg;
	use diesel::expression::AsExpression;

	use crate::database::get_db_conn;

	// Define pg_trgm operators
	// Functions do not use indices for queries, so we need to use operators
	diesel::infix_operator!(Similarity, " % ", backend: Pg);
	diesel::infix_operator!(Distance, " <-> ", Float, backend: Pg);

	// Create functions to make use of the operators in queries
	fn trgm_similar<T: AsExpression<Text>, U: AsExpression<Text>>(left: T, right: U)
		-> Similarity<T::Expression, U::Expression> {
		Similarity::new(left.as_expression(), right.as_expression())
	}

	fn trgm_distance<T: AsExpression<Text>, U: AsExpression<Text>>(left: T, right: U)
		-> Distance<T::Expression, U::Expression> {
		Distance::new(left.as_expression(), right.as_expression())
	}
}
}

/// Search for albums by title
/// 
/// # Arguments
/// `query` - The search query. This will be used to perform a fuzzy search on the album titles
/// `limit` - The maximum number of results to return
/// 
/// # Returns
/// A Result containing a vector of albums if the search was successful, or an error if the search failed
#[server(endpoint = "search_albums")]
pub async fn search_albums(query: String, limit: i64) -> Result<Vec<Album>, ServerFnError> {
	use crate::schema::albums::dsl::*;

	Ok(albums
		.filter(trgm_similar(title, query.clone()))
		.order_by(trgm_distance(title, query))
		.limit(limit)
		.load(&mut get_db_conn())?)
}

/// Search for artists by name
/// 
/// # Arguments
/// `query` - The search query. This will be used to perform a fuzzy search on the artist names
/// `limit` - The maximum number of results to return
/// 
/// # Returns
/// A Result containing a vector of artists if the search was successful, or an error if the search failed
#[server(endpoint = "search_artists")]
pub async fn search_artists(query: String, limit: i64) -> Result<Vec<Artist>, ServerFnError> {
	use crate::schema::artists::dsl::*;

	Ok(artists
		.filter(trgm_similar(name, query.clone()))
		.order_by(trgm_distance(name, query))
		.limit(limit)
		.load(&mut get_db_conn())?)
}

/// Search for songs by title
/// 
/// # Arguments
/// `query` - The search query. This will be used to perform a fuzzy search on the song titles
/// `limit` - The maximum number of results to return
/// 
/// # Returns
/// A Result containing a vector of songs if the search was successful, or an error if the search failed
#[server(endpoint = "search_songs")]
pub async fn search_songs(query: String, limit: i64) -> Result<Vec<Song>, ServerFnError> {
	use crate::schema::songs::dsl::*;

	Ok(songs
		.filter(trgm_similar(title, query.clone()))
		.order_by(trgm_distance(title, query))
		.limit(limit)
		.load(&mut get_db_conn())?)
}

/// Search for songs, albums, and artists by title or name
/// 
/// # Arguments
/// `query` - The search query. This will be used to perform a fuzzy search on the
/// song titles, album titles, and artist names
/// `limit` - The maximum number of results to return for each type
/// 
/// # Returns
/// A Result containing a tuple of vectors of albums, artists, and songs if the search was successful,
#[server(endpoint = "search")]
pub async fn search(query: String, limit: i64) -> Result<(Vec<Album>, Vec<Artist>, Vec<Song>), ServerFnError> {
	let albums = search_albums(query.clone(), limit);
	let artists = search_artists(query.clone(), limit);
	let songs = search_songs(query.clone(), limit);

	use tokio::join;

	let (albums, artists, songs) = join!(albums, artists, songs);
	Ok((albums?, artists?, songs?))
}
