use serde::{Deserialize, Serialize};

/// Differentiates between different types of media
/// Used to display a short text near a corresponging image / title to indicate what type of media it is
#[derive(Serialize, Deserialize)]
pub enum MediaType {
	Song,
	Album,
	Artist,
}

impl ToString for MediaType {
	fn to_string(&self) -> String {
		match self {
			MediaType::Song => "Song".to_string(),
			MediaType::Album => "Album".to_string(),
			MediaType::Artist => "Artist".to_string(),
		}
	}
}
