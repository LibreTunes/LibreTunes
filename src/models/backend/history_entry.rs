use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
	}
}

/// Model for a history entry
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::song_history))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize)]
pub struct HistoryEntry {
	/// A unique id for the history entry
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The id of the user who listened to the song
	pub user_id: i32,
	/// The date the song was listened to
	pub date: NaiveDateTime,
	/// The id of the song that was listened to
	pub song_id: i32,
}
