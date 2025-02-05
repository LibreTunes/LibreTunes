use crate::components::dashboard_tile::DashboardTile;
use serde::{Serialize, Deserialize};

/// Holds information about an artist
/// 
/// Intended to be used in the front-end
#[derive(Clone, Serialize, Deserialize)]
pub struct Artist {
	/// Artist id
	pub id: i32,
	/// Artist name
	pub name: String,
	/// Path to artist image, relative to the root of the web server.
	/// For example, `"/assets/images/Artist.jpg"`
	pub image_path: String,
}

impl Into<DashboardTile> for Artist {
	fn into(self) -> DashboardTile {
		DashboardTile {
			image_path: self.image_path.into(),
			title: self.name.into(),
			link: format!("/artist/{}", self.id).into(),
			description: Some("Artist".into()),
		}
	}
}
