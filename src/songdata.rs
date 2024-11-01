use crate::models::{Album, Artist, Song};
use crate::components::dashboard_tile::DashboardTile;

use serde::{Serialize, Deserialize};
use time::Date;

/// Holds information about a song
/// 
/// Intended to be used in the front-end, as it includes artist and album objects, rather than just their ids.
#[derive(Serialize, Deserialize, Clone)]
pub struct SongData {
	/// Song id
	pub id: i32,
	/// Song name
	pub title: String,
	/// Song artists
	pub artists: Vec<Artist>,
	/// Song album
	pub album: Option<Album>,
	/// The track number of the song on the album
	pub track: Option<i32>,
	/// The duration of the song in seconds
	pub duration: i32,
	/// The song's release date
	pub release_date: Option<Date>,
	/// Path to song file, relative to the root of the web server.
	/// For example, `"/assets/audio/Song.mp3"`
	pub song_path: String,
	/// Path to song image, relative to the root of the web server.
	/// For example, `"/assets/images/Song.jpg"`
	pub image_path: String,
	/// Whether the song is liked by the user
	pub like_dislike: Option<(bool, bool)>,
}


impl TryInto<Song> for SongData {
	type Error = Box<dyn std::error::Error>;

	/// Convert a SongData object into a Song object
	/// 
	/// The SongData/Song conversions are also not truly reversible,
	/// due to the way the image_path data is handled.
	fn try_into(self) -> Result<Song, Self::Error> {
		Ok(Song {
			id: Some(self.id),
			title: self.title,
			album_id: self.album.map(|album|
				album.id.ok_or("Album id must be present (Some) to convert to Song")).transpose()?,
			track: self.track,
			duration: self.duration,
			release_date: self.release_date,
			// TODO https://gitlab.mregirouard.com/libretunes/libretunes/-/issues/35
			storage_path: self.song_path,

			// Note that if the source of the image_path was the album, the image_path
			// will be set to the album's image_path instead of None
			image_path: if self.image_path == "/assets/images/placeholder.jpg" {
				None
			} else {
				Some(self.image_path)
			},
		})
	}
}

impl DashboardTile for SongData {
	fn image_path(&self) -> String {
		self.image_path.clone()
	}

	fn title(&self) -> String {
		self.title.clone()
	}

	fn link(&self) -> String {
		format!("/song/{}", self.id)
	}

	fn description(&self) -> Option<String> {
		Some(format!("Song • {}", Artist::display_list(&self.artists)))
	}
}
