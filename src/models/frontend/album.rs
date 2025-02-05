use crate::models::backend::Artist;
use crate::components::dashboard_tile::DashboardTile;
use serde::{Serialize, Deserialize};

use chrono::NaiveDate;

/// Holds information about an album
/// 
/// Intended to be used in the front-end

#[derive(Serialize, Deserialize, Clone)]
pub struct Album {
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

impl Into<DashboardTile> for Album {
	fn into(self) -> DashboardTile {
		DashboardTile {
			image_path: self.image_path.into(),
			title: self.title.into(),
			link: format!("/album/{}", self.id).into(),
			description: Some(format!("Album • {}", Artist::display_list(&self.artists)).into()),
		}
	}
}
