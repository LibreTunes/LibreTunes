use leptos::leptos_dom::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use crate::media_type::MediaType;

/// Info representing what will be displayed in a dashboard tile
#[derive(Serialize, Deserialize)]
pub struct DashboardTile {
	pub image_path: String,
	pub title: String,
	pub media_type: Option<MediaType>,
	pub artist: Option<String>,
}

impl DashboardTile {
	pub fn new(image_path: String, title: String, media_type: Option<MediaType>, artist: Option<String>) -> Self {
		Self {
			image_path,
			title,
			media_type,
			artist: artist.map(|artist| artist.to_string()),
		}
	}

	/// Get the description of the dashboard tile
	/// Will display the media type, and the artist if it is available and relevant
	pub fn description(&self) -> String {
		match self.media_type {
			Some(MediaType::Song) => {
				if let Some(artist) = &self.artist {
					format!("{} • {}", MediaType::Song.to_string(), artist)
				} else {
					MediaType::Song.to_string()
				}
			},
			Some(MediaType::Album) => {
				if let Some(artist) = &self.artist {
					format!("{} • {}", MediaType::Album.to_string(), artist)
				} else {
					MediaType::Album.to_string()
				}
			},
			Some(MediaType::Artist) => {
				MediaType::Artist.to_string()
			},
			None => {
				if let Some(artist) = &self.artist {
					artist.to_string()
				} else {
					"".to_string()
				}
			}
		}
	}
}

impl IntoView  for DashboardTile {
	fn into_view(self) -> View {
		let description = self.description();

		view! {
			<div class="dashboard-tile">
				<img src={self.image_path} alt="dashboard-tile" />
				<p class="dashboard-tile-title">{self.title}</p>
				<p class="dashboard-tile-description">{description}</p>
			</div>
		}.into_view()
	}
}
