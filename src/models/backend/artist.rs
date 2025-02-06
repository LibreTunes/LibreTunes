use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use diesel::prelude::*;
	}
}

/// Model for an artist
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable, Identifiable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::artists))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artist {
	/// A unique id for the artist
	#[cfg_attr(feature = "ssr", diesel(deserialize_as = i32))]
	pub id: Option<i32>,
	/// The artist's name
	pub name: String,
}

impl Artist {
	/// Display a list of artists as a string.
	/// 
	/// For one artist, displays [artist1]. For two artists, displays [artist1] & [artist2].
	/// For three or more artists, displays [artist1], [artist2], & [artist3].
	pub fn display_list(artists: &[Artist]) -> String {
		let mut artist_list = String::new();

		for (i, artist) in artists.iter().enumerate() {
			if i == 0 {
				artist_list.push_str(&artist.name);
			} else if i == artists.len() - 1 {
				artist_list.push_str(&format!(" & {}", artist.name));
			} else {
				artist_list.push_str(&format!(", {}", artist.name));
			}
		}

		artist_list
	}
}
