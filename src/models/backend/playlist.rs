use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
	}
}

/// Model for a playlist
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::playlists))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize)]
pub struct Playlist {
	/// A unique id for the playlist
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The time the playlist was created
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = NaiveDateTime))]
	pub created_at: Option<NaiveDateTime>,
	/// The time the playlist was last updated
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = NaiveDateTime))]
	pub updated_at: Option<NaiveDateTime>,
	/// The id of the user who owns the playlist
	pub owner_id: i32,
	/// The name of the playlist
	pub name: String,
}
