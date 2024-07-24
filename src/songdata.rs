use crate::database;
use crate::models::{Album, Artist, Song};

use time::Date;

/// Holds information about a song
/// 
/// Intended to be used in the front-end, as it includes artist and album objects, rather than just their ids.
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
}

#[cfg(feature = "ssr")]
impl TryInto<SongData> for Song {
	type Error = Box<dyn std::error::Error>;

	/// Convert a Song object into a SongData object
	/// 
	/// This conversion is expensive, as it requires database queries to get the artist and album objects.
	/// The SongData/Song conversions are also not truly reversible,
	/// due to the way the image_path, album, and artist data is handled.
	fn try_into(self) -> Result<SongData, Self::Error> {
		let mut db_con = database::get_db_conn();

		let album = self.get_album(&mut db_con)?;

		// Use the song's image path if it exists, otherwise use the album's image path, or fallback to the placeholder
		let image_path = self.image_path.clone().unwrap_or_else(|| {
			album
				.as_ref()
				.and_then(|album| album.image_path.clone())
				.unwrap_or_else(|| "/assets/images/placeholder.jpg".to_string())
		});

		Ok(SongData {
			id: self.id.ok_or("Song id must be present (Some) to convert to SongData")?,
			title: self.title.clone(),
			artists: self.get_artists(&mut db_con)?,
			album: album,
			track: self.track,
			duration: self.duration,
			release_date: self.release_date,
			// TODO https://gitlab.mregirouard.com/libretunes/libretunes/-/issues/35
			song_path: self.storage_path,
			image_path: image_path,
		})
	}
}

impl TryInto<Song> for SongData {
	type Error = Box<dyn std::error::Error>;

	/// Convert a SongData object into a Song object
	/// 
	/// The SongData/Song conversions are also not truly reversible,
	/// due to the way the image_path, album, and  and artist data is handled.
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
