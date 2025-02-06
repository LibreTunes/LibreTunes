use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
	}
}

/// Model for a song
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::songs))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Clone, Serialize, Deserialize)]
pub struct Song {
	/// A unique id for the song
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The song's title
	pub title: String,
	/// The album the song is from
	pub album_id: Option<i32>,
	/// The track number of the song on the album
	pub track: Option<i32>,
	/// The duration of the song in seconds
	pub duration: i32,
	/// The song's release date
	pub release_date: Option<NaiveDate>,
	/// The path to the song's audio file
	pub storage_path: String,
	/// The path to the song's image file
	pub image_path: Option<String>,
	/// The date the song was added to the database
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = NaiveDateTime))]
	pub added_date: Option<NaiveDateTime>,
}
