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

impl From<Artist> for DashboardTile {
	fn from(val: Artist) -> Self {
		DashboardTile {
			image_path: val.image_path.into(),
			title: val.name.into(),
			link: format!("/artist/{}", val.id).into(),
			description: Some("Artist".into()),
		}
	}
}
