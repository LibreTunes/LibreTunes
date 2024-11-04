use crate::components::dashboard_tile::DashboardTile;
use serde::{Serialize, Deserialize};

/// Holds information about an artist
/// 
/// Intended to be used in the front-end
#[derive(Clone, Serialize, Deserialize)]
pub struct ArtistData {
	/// Artist id
	pub id: i32,
	/// Artist name
	pub name: String,
	/// Path to artist image, relative to the root of the web server.
	/// For example, `"/assets/images/Artist.jpg"`
	pub image_path: String,
}

impl DashboardTile for ArtistData {
	fn image_path(&self) -> String {
		self.image_path.clone()
	}

	fn title(&self) -> String {
		self.name.clone()
	}

	fn link(&self) -> String {
		format!("/artist/{}", self.id)
	}

	fn description(&self) -> Option<String> {
		Some("Artist".to_string())
	}
}
