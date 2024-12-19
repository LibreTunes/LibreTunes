use crate::models::Artist;
use crate::components::dashboard_tile::DashboardTile;
use serde::{Serialize, Deserialize};

use chrono::NaiveDate;

/// Holds information about an album
/// 
/// Intended to be used in the front-end

#[derive(Serialize, Deserialize, Clone)]
pub struct AlbumData {
	/// Album id
	pub id: i32,
	/// Album title
	pub title: String,
	/// Album artists
	pub artists: Vec<Artist>,
	/// Album release date
	pub release_date: Option<NaiveDate>,
	/// Path to album image, relative to the root of the web server.
	/// For example, `"/assets/images/Album.jpg"`
	pub image_path: String,
}

impl DashboardTile for AlbumData {
	fn image_path(&self) -> String {
		self.image_path.clone()
	}

	fn title(&self) -> String {
		self.title.clone()
	}

	fn link(&self) -> String {
		format!("/album/{}", self.id)
	}

	fn description(&self) -> Option<String> {
		Some(format!("Album • {}", Artist::display_list(&self.artists)))
	}
}